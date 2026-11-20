//! An order-entry tester for a FIX 4.2 venue: logs on, enters the orders a JSON file lists, shows
//! every ExecutionReport and OrderCancelReject as it arrives, and cancels orders.
//!
//! ```sh
//! cargo run --example gateway --all-features -- --allow CLIENT1     # a venue to try it on
//! cargo run --example fix-tester -- --field 6751=TRADER1 < orders.json
//! ```
//!
//! The orders are a JSON array, read from `--orders FILE` or, by default, from stdin:
//!
//! ```json
//! [
//!   {"symbol": "SHOP", "side": "buy", "qty": 100, "price": "85.10", "ex_destination": "ASPC"},
//!   {"cl_ord_id": "mid-1", "symbol": "RY", "side": "sell", "qty": 500, "ord_type": "pegged",
//!    "time_in_force": "ioc", "ex_destination": "INCC", "fields": {"18": "M", "110": "200"}}
//! ]
//! ```
//!
//! `side` is buy, sell or sell_short; `ord_type` market, limit or pegged (default limit if there's
//! a price, else market); `time_in_force` day, ioc, gtc or gtd (default day). `cl_ord_id` defaults
//! to one made up for this run. `fields` adds any other tags, such as a venue's own (CIX's
//! UMIRUserID(6751), ExecInst(18) for its pegs, ExpireTime(126) for GTT); `--field TAG=VALUE` adds
//! one to every order.
//!
//! On a terminal it shows the orders in a table above a log of updates: ↑/↓ (or j/k) select an
//! order, `c` cancels it, `a` cancels every open order, `q` or Ctrl-C logs out. Otherwise (stdout
//! piped to a file, say) it prints the updates one per line, and Ctrl-C logs out.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::error::Error;
use std::io::{IsTerminal, Read, Write};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ratatui::crossterm::event::{self, Event as TermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListItem, Row, Table, TableState};
use ratatui::{DefaultTerminal, Frame};
use serde::Deserialize;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;
use turbojet::fields::{Code, Decimal, FixEnum, UtcTimestamp};
use turbojet::{
    Application, Context, Disconnect, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_fix::fix42::{
    ExecType, ExecutionReport, HandlInst, NewOrderSingle, NewOrderSingleRequired, OrdStatus, OrdType,
    OrderCancelReject, OrderCancelRequest, OrderCancelRequestRequired, Side, TimeInForce,
};

/// More than anyone would type in by hand; a bound on the file's size in memory.
const MAX_ORDERS: usize = 10_000;
/// Log lines kept for the terminal view: far more than a screen holds.
const MAX_LOG_LINES: usize = 1_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let specs = read_orders(args.orders.as_deref())?;
    let tui = std::io::stdout().is_terminal();
    let (tx, mut events) = mpsc::unbounded_channel();
    init_tracing(tui, tx.clone());

    let mut config =
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", args.sender), args.target).with_endpoint(args.addr.as_str());
    config.reset_on_logon = true;
    let app = Arc::new(TesterApp { events: tx.clone() });
    let initiator = Initiator::new(config, Arc::new(MemoryStorage::new()), app)?;
    let session = initiator.handle();
    let mut connection = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.connect_once().await }
    });
    tokio::select! {
        event = tokio::time::timeout(Duration::from_secs(10), events.recv()) => match event {
            Ok(Some(Event::LoggedOn)) => {}
            _ => return Err(format!("logon to {} failed", args.addr).into()),
        },
        result = &mut connection => return Err(format!("connection ended before logon: {:?}", result?).into()),
    }

    let mut blotter = Blotter::new(format!("{} at {}", session.id(), args.addr));
    blotter.enter_orders(&session, &specs, &args.fields)?;
    if tui {
        spawn_key_reader(tx);
        let mut terminal = ratatui::init();
        let result = run_tui(&mut terminal, &session, &mut blotter, &mut events).await;
        ratatui::restore();
        result?;
    } else {
        run_plain(&session, &mut blotter, &mut events).await?;
    }
    // The connection task ends once the Logout exchange (or its timeout) is over.
    connection.await??;
    Ok(())
}

/// Shows the blotter on the terminal until the session logs out.
async fn run_tui(
    terminal: &mut DefaultTerminal,
    session: &SessionHandle,
    blotter: &mut Blotter,
    events: &mut mpsc::UnboundedReceiver<Event>,
) -> Result<(), Box<dyn Error>> {
    let mut selected = TableState::default().with_selected(0);
    loop {
        terminal.draw(|frame| draw(frame, blotter, &mut selected))?;
        match events.recv().await {
            Some(Event::Message(msg)) => blotter.on_message(&msg)?,
            Some(Event::Log(line)) => blotter.note(line),
            Some(Event::Key(key)) => match key.code {
                KeyCode::Char('c' | 'C') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    logout(session, blotter)?;
                }
                KeyCode::Up | KeyCode::Char('k') => selected.select_previous(),
                KeyCode::Down | KeyCode::Char('j') => selected.select_next(),
                KeyCode::Char('c') => {
                    if let Some(index) = selected.selected().filter(|&i| i < blotter.orders.len()) {
                        blotter.cancel(session, index)?;
                    }
                }
                KeyCode::Char('a') => blotter.cancel_all(session)?,
                KeyCode::Char('q') => logout(session, blotter)?,
                _ => {}
            },
            Some(Event::LoggedOn) => {}
            Some(Event::LoggedOut) | None => return Ok(()),
        }
    }
}

/// Prints the updates as they come until Ctrl-C, for when stdout isn't a terminal.
async fn run_plain(
    session: &SessionHandle,
    blotter: &mut Blotter,
    events: &mut mpsc::UnboundedReceiver<Event>,
) -> Result<(), Box<dyn Error>> {
    loop {
        for line in blotter.log.drain(..) {
            println!("{line}");
        }
        tokio::select! {
            event = events.recv() => match event {
                Some(Event::Message(msg)) => blotter.on_message(&msg)?,
                Some(Event::Log(line)) => eprint!("{line}"),
                Some(Event::Key(_) | Event::LoggedOn) => {}
                Some(Event::LoggedOut) | None => return Ok(()),
            },
            _ = tokio::signal::ctrl_c() => logout(session, blotter)?,
        }
    }
}

fn logout(session: &SessionHandle, blotter: &mut Blotter) -> Result<(), Box<dyn Error>> {
    blotter.note("logging out".into());
    session.logout(None)?;
    Ok(())
}

fn draw(frame: &mut Frame, blotter: &Blotter, selected: &mut TableState) {
    let [table_area, log_area, help_area] =
        Layout::vertical([Constraint::Percentage(55), Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

    const HEADER: [&str; 10] =
        ["ClOrdID", "Symbol", "Side", "Type", "Qty", "Price", "Status", "Leaves", "Cum", "AvgPx"];
    let cells: Vec<[String; 10]> = blotter
        .orders
        .iter()
        .map(|order| {
            [
                order.label.clone(),
                order.symbol.clone(),
                name(&Code::from(order.side)),
                name(&Code::from(order.ord_type)),
                order.qty.to_string(),
                order.price.map_or_else(String::new, |price| price.to_string()),
                order.status.clone(),
                order.leaves_qty.to_string(),
                order.cum_qty.to_string(),
                order.avg_px.to_string(),
            ]
        })
        .collect();
    // Each column as wide as its widest cell, so nothing is cut short while the screen has room.
    let widths = (0..HEADER.len()).map(|column| {
        let widest = cells.iter().map(|row| row[column].chars().count()).max().unwrap_or(0);
        Constraint::Length(u16::try_from(widest.max(HEADER[column].len())).unwrap_or(u16::MAX))
    });
    let rows = blotter
        .orders
        .iter()
        .zip(cells.iter())
        .map(|(order, row)| Row::new(row.clone()).style(Style::new().fg(status_color(&order.status))));
    let header = Row::new(HEADER).style(Style::new().bold().underlined());
    let open = blotter.orders.iter().filter(|order| !order.done).count();
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::bordered()
                .title(format!(" {} ", blotter.title))
                .title_bottom(format!(" {} orders, {open} open ", blotter.orders.len())),
        )
        .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");
    frame.render_stateful_widget(table, table_area, selected);

    // The newest lines that fit, oldest at the top.
    let height = usize::from(log_area.height.saturating_sub(2));
    let items = blotter.log.iter().skip(blotter.log.len().saturating_sub(height)).map(|line| {
        let style = if line.starts_with("->") { Style::new().fg(Color::Cyan) } else { Style::new() };
        ListItem::new(line.as_str()).style(style)
    });
    frame.render_widget(List::new(items).block(Block::bordered().title(" Log ")), log_area);

    let help = Line::from(" ↑/↓ select   c cancel   a cancel all   q quit").style(Style::new().dark_gray());
    frame.render_widget(help, help_area);
}

fn status_color(status: &str) -> Color {
    match status {
        "New" => Color::Green,
        "PartiallyFilled" => Color::Yellow,
        "Filled" => Color::LightBlue,
        "Rejected" => Color::Red,
        "Canceled" | "Expired" => Color::DarkGray,
        _ => Color::Reset,
    }
}

/// One order, as the JSON file gives it.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderSpec {
    cl_ord_id: Option<String>,
    symbol: String,
    side: SideName,
    #[serde(deserialize_with = "decimal")]
    qty: Decimal,
    #[serde(default, deserialize_with = "optional_decimal")]
    price: Option<Decimal>,
    ord_type: Option<OrdTypeName>,
    time_in_force: Option<TimeInForceName>,
    ex_destination: Option<String>,
    account: Option<String>,
    #[serde(default)]
    fields: BTreeMap<u32, String>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum SideName {
    Buy,
    Sell,
    SellShort,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum OrdTypeName {
    Market,
    Limit,
    Pegged,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TimeInForceName {
    Day,
    Ioc,
    Gtc,
    Gtd,
}

/// A quantity or price, written either as a JSON number or a string. Parsed from its text, so a
/// price like 85.10 isn't rounded through a float.
fn decimal<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Decimal, D::Error> {
    let text = match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(text) => text,
        serde_json::Value::Number(number) => number.to_string(),
        other => return Err(serde::de::Error::custom(format!("expected a number, got {other}"))),
    };
    text.parse().map_err(|_| serde::de::Error::custom(format!("'{text}' isn't a decimal")))
}

fn optional_decimal<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<Decimal>, D::Error> {
    decimal(deserializer).map(Some)
}

impl OrderSpec {
    fn side(&self) -> Side {
        match self.side {
            SideName::Buy => Side::Buy,
            SideName::Sell => Side::Sell,
            SideName::SellShort => Side::SellShort,
        }
    }

    fn ord_type(&self) -> OrdType {
        match self.ord_type {
            Some(OrdTypeName::Market) => OrdType::Market,
            Some(OrdTypeName::Limit) => OrdType::Limit,
            Some(OrdTypeName::Pegged) => OrdType::Pegged,
            None if self.price.is_some() => OrdType::Limit,
            None => OrdType::Market,
        }
    }

    fn new_order(&self, cl_ord_id: &str, extra: &[(u32, String)]) -> Message {
        let mut order = NewOrderSingle::new(NewOrderSingleRequired {
            cl_ord_id: cl_ord_id.into(),
            handl_inst: HandlInst::AutomatedExecutionNoIntervention.into(),
            symbol: self.symbol.as_str().into(),
            side: self.side().into(),
            transact_time: UtcTimestamp::now(),
            ord_type: self.ord_type().into(),
        });
        order.order_qty = Some(self.qty);
        order.price = self.price;
        order.time_in_force = Some(
            match self.time_in_force.unwrap_or(TimeInForceName::Day) {
                TimeInForceName::Day => TimeInForce::Day,
                TimeInForceName::Ioc => TimeInForce::ImmediateOrCancel,
                TimeInForceName::Gtc => TimeInForce::GoodTillCancel,
                TimeInForceName::Gtd => TimeInForce::GoodTillDate,
            }
            .into(),
        );
        order.ex_destination = self.ex_destination.as_deref().map(Into::into);
        order.account = self.account.as_deref().map(Into::into);
        let mut msg = Message::from(order);
        // Per-order fields win over the --field ones for the same tag.
        for (tag, value) in extra.iter().filter(|(tag, _)| !self.fields.contains_key(tag)) {
            msg.set(*tag, value.as_str());
        }
        for (tag, value) in &self.fields {
            msg.set(*tag, value.as_str());
        }
        msg
    }
}

/// An order this run entered, and what the venue last said about it.
struct Order {
    /// The ClOrdID it was entered with: what the table and log call it.
    label: String,
    /// The ClOrdID the venue last confirmed, which a cancel names as its OrigClOrdID.
    cl_ord_id: String,
    symbol: String,
    side: Side,
    ord_type: OrdType,
    qty: Decimal,
    price: Option<Decimal>,
    status: String,
    leaves_qty: Decimal,
    cum_qty: Decimal,
    avg_px: Decimal,
    done: bool,
    cancels_sent: u32,
}

/// The orders, found by any ClOrdID they or their cancels have had, and a log of what happened.
struct Blotter {
    title: String,
    orders: Vec<Order>,
    by_cl_ord_id: HashMap<String, usize>,
    log: VecDeque<String>,
}

impl Blotter {
    fn new(title: String) -> Self {
        Self { title, orders: Vec::new(), by_cl_ord_id: HashMap::new(), log: VecDeque::new() }
    }

    fn note(&mut self, line: String) {
        if self.log.len() == MAX_LOG_LINES {
            self.log.pop_front();
        }
        self.log.push_back(line);
    }

    /// Enters every order, giving those without a ClOrdID one unique to this run.
    fn enter_orders(
        &mut self,
        session: &SessionHandle,
        specs: &[OrderSpec],
        extra: &[(u32, String)],
    ) -> Result<(), Box<dyn Error>> {
        let run = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        for (n, spec) in specs.iter().enumerate() {
            let cl_ord_id = spec.cl_ord_id.clone().unwrap_or_else(|| format!("{run}-{}", n + 1));
            if self.by_cl_ord_id.insert(cl_ord_id.clone(), n).is_some() {
                return Err(format!("ClOrdID {cl_ord_id} appears twice").into());
            }
            let msg = spec.new_order(&cl_ord_id, extra);
            self.note(format!("-> {cl_ord_id}: {msg}"));
            session.send(msg)?;
            self.orders.push(Order {
                label: cl_ord_id.clone(),
                cl_ord_id,
                symbol: spec.symbol.clone(),
                side: spec.side(),
                ord_type: spec.ord_type(),
                qty: spec.qty,
                price: spec.price,
                status: "Sent".into(),
                leaves_qty: spec.qty,
                cum_qty: Decimal::ZERO,
                avg_px: Decimal::ZERO,
                done: false,
                cancels_sent: 0,
            });
        }
        Ok(())
    }

    fn find(&self, cl_ord_id: Option<&str>) -> Option<usize> {
        cl_ord_id.and_then(|id| self.by_cl_ord_id.get(id).copied())
    }

    fn on_message(&mut self, msg: &Message) -> Result<(), Box<dyn Error>> {
        match msg.msg_type() {
            MsgType::ExecutionReport => self.on_execution_report(&msg.parse()?),
            MsgType::OrderCancelReject => {
                let reject: OrderCancelReject = msg.parse()?;
                let label = self.find(Some(&reject.orig_cl_ord_id)).map_or("?", |i| &self.orders[i].label);
                let line = format!(
                    "<- {label}: cancel {} rejected, status {}{}{}",
                    reject.cl_ord_id,
                    name(&reject.ord_status),
                    reject
                        .cxl_rej_reason
                        .as_ref()
                        .map_or_else(String::new, |reason| format!(" reason {}", name(reason))),
                    reject.text.as_deref().map_or_else(String::new, |text| format!(" \"{text}\"")),
                );
                self.note(line);
            }
            _ => self.note(format!("<- {msg}")),
        }
        Ok(())
    }

    fn on_execution_report(&mut self, report: &ExecutionReport) {
        let found = self.find(report.cl_ord_id.as_deref()).or_else(|| self.find(report.orig_cl_ord_id.as_deref()));
        let Some(index) = found else {
            self.note(format!("<- ExecutionReport for an order this run didn't enter: {:?}", report.cl_ord_id));
            return;
        };
        let order = &mut self.orders[index];
        if matches!(report.exec_type.known(), Some(ExecType::Canceled | ExecType::Replaced))
            && let Some(cl_ord_id) = &report.cl_ord_id
        {
            order.cl_ord_id = cl_ord_id.to_string();
        }
        order.status = name(&report.ord_status);
        order.leaves_qty = report.leaves_qty;
        order.cum_qty = report.cum_qty;
        order.avg_px = report.avg_px;
        order.done = matches!(
            report.ord_status.known(),
            Some(OrdStatus::Filled | OrdStatus::Canceled | OrdStatus::Rejected | OrdStatus::Expired)
        );
        let fill = match (report.last_shares, report.last_px) {
            (Some(shares), Some(px)) if !shares.is_zero() => format!(" filled {shares}@{px}"),
            _ => String::new(),
        };
        let line = format!(
            "<- {}: {} -> {}{fill} leaves {} cum {}{}{}",
            order.label,
            name(&report.exec_type),
            order.status,
            order.leaves_qty,
            order.cum_qty,
            report.ord_rej_reason.as_ref().map_or_else(String::new, |reason| format!(" reason {}", name(reason))),
            report.text.as_deref().map_or_else(String::new, |text| format!(" \"{text}\"")),
        );
        self.note(line);
    }

    fn cancel(&mut self, session: &SessionHandle, index: usize) -> Result<(), Box<dyn Error>> {
        let order = &mut self.orders[index];
        if order.done {
            let line = format!("{} is already {}", order.label, order.status);
            self.note(line);
            return Ok(());
        }
        order.cancels_sent += 1;
        let cl_ord_id = format!("{}-C{}", order.label, order.cancels_sent);
        let mut request = OrderCancelRequest::new(OrderCancelRequestRequired {
            orig_cl_ord_id: order.cl_ord_id.as_str().into(),
            cl_ord_id: cl_ord_id.as_str().into(),
            symbol: order.symbol.as_str().into(),
            side: order.side.into(),
            transact_time: UtcTimestamp::now(),
        });
        request.order_qty = Some(order.qty);
        let mut msg = Message::from(request);
        // Not a FIX 4.2 OrderCancelRequest field, but some venues (CIX among them) require it.
        msg.set(40, order.ord_type.code());
        let line = format!("-> {}: cancel {cl_ord_id}", order.label);
        session.send(msg)?;
        self.note(line);
        self.by_cl_ord_id.insert(cl_ord_id, index);
        Ok(())
    }

    fn cancel_all(&mut self, session: &SessionHandle) -> Result<(), Box<dyn Error>> {
        for index in 0..self.orders.len() {
            if !self.orders[index].done {
                self.cancel(session, index)?;
            }
        }
        Ok(())
    }
}

/// An enum value's name, or its code if this FIX version doesn't list it.
fn name<E: FixEnum + std::fmt::Debug>(code: &Code<E>) -> String {
    code.known().map_or_else(|| code.code().to_string(), |value| format!("{value:?}"))
}

/// What `main` waits on: the session, the keyboard and the engine's log, in one queue.
enum Event {
    LoggedOn,
    LoggedOut,
    Message(Message),
    Key(KeyEvent),
    Log(String),
}

/// Forwards engine callbacks to `main`.
struct TesterApp {
    events: mpsc::UnboundedSender<Event>,
}

impl Application for TesterApp {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.events.send(Event::LoggedOn);
    }

    fn on_logout(&self, _session: &SessionHandle, _ended: Disconnect) {
        let _ = self.events.send(Event::LoggedOut);
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(Event::Message(msg.clone()));
        Ok(())
    }
}

/// Reads key presses on a thread of their own, since the read blocks. Crossterm reads the
/// terminal itself, not stdin, so this works with the orders piped in.
fn spawn_key_reader(events: mpsc::UnboundedSender<Event>) {
    std::thread::spawn(move || {
        while let Ok(event) = event::read() {
            if let TermEvent::Key(key) = event
                && key.kind == KeyEventKind::Press
                && events.send(Event::Key(key)).is_err()
            {
                break;
            }
        }
    });
}

/// Sends the engine's log lines to the log panel (on a terminal, where writing them to stderr
/// would draw over the view) or to stderr.
fn init_tracing(tui: bool, events: mpsc::UnboundedSender<Event>) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let builder = tracing_subscriber::fmt().with_env_filter(filter).without_time();
    if tui {
        builder.with_ansi(false).with_writer(move || LogWriter(events.clone())).init();
    } else {
        builder.with_writer(std::io::stderr).init();
    }
}

struct LogWriter(mpsc::UnboundedSender<Event>);

impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let line = String::from_utf8_lossy(buf).trim_end().to_string();
        let _ = self.0.send(Event::Log(line));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct Args {
    addr: String,
    sender: String,
    target: String,
    orders: Option<String>,
    fields: Vec<(u32, String)>,
}

const USAGE: &str = "\
Usage: fix-tester [ADDR] [--orders FILE] [--sender ID] [--target ID] [--field TAG=VALUE]...

  ADDR               The venue's address (default 127.0.0.1:9876)
  --orders FILE      Read the orders from FILE (default: stdin)
  --sender ID        Our SenderCompID (default CLIENT1)
  --target ID        The venue's CompID (default GATEWAY)
  --field TAG=VALUE  Add a field to every order, e.g. --field 6751=TRADER1";

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut parsed = Args {
        addr: "127.0.0.1:9876".into(),
        sender: "CLIENT1".into(),
        target: "GATEWAY".into(),
        orders: None,
        fields: Vec::new(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} requires a value\n\n{USAGE}"));
        match arg.as_str() {
            "--orders" => parsed.orders = Some(value()?),
            "--sender" => parsed.sender = value()?,
            "--target" => parsed.target = value()?,
            "--field" => {
                let field = value()?;
                let (tag, value) = field.split_once('=').ok_or(format!("--field {field}: expected TAG=VALUE"))?;
                parsed.fields.push((tag.parse().map_err(|_| format!("--field {field}: bad tag"))?, value.into()));
            }
            "-h" | "--help" => return Err(USAGE.into()),
            addr if !addr.starts_with('-') => parsed.addr = addr.into(),
            other => return Err(format!("unknown argument '{other}'\n\n{USAGE}").into()),
        }
    }
    Ok(parsed)
}

fn read_orders(path: Option<&str>) -> Result<Vec<OrderSpec>, Box<dyn Error>> {
    let json = match path {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut json = String::new();
            std::io::stdin().read_to_string(&mut json)?;
            json
        }
    };
    let orders: Vec<OrderSpec> = serde_json::from_str(&json)?;
    if orders.len() > MAX_ORDERS {
        return Err(format!("{} orders; at most {MAX_ORDERS}", orders.len()).into());
    }
    Ok(orders)
}

//! Both roles in one program: an acceptor acknowledges orders as `SERVER`, and an initiator logs
//! on to it as `CLIENT`, sends an order and waits for the acknowledgement.

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::fields::{Decimal, UtcTimestamp};
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_fix::fix44::{
    ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingle, NewOrderSingleRef, NewOrderSingleRequired,
    OrdStatus, OrdType, Side,
};

/// Acknowledges every order.
struct OrderDesk;

impl Application for OrderDesk {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        match msg.msg_type() {
            MsgType::NewOrderSingle => {
                // A missing or malformed field becomes a Reject(3) naming the tag.
                let order: NewOrderSingleRef = msg.parse()?;
                let mut ack = ExecutionReport::new(ExecutionReportRequired {
                    order_id: "O1".into(),
                    exec_id: "E1".into(),
                    exec_type: ExecType::New.into(),
                    ord_status: OrdStatus::New.into(),
                    side: order.side.into(),
                    leaves_qty: order.order_qty.unwrap_or_default(),
                    cum_qty: Decimal::ZERO,
                    avg_px: Decimal::ZERO,
                });
                ack.cl_ord_id = Some(order.cl_ord_id.into());
                ctx.send(ack);
                Ok(())
            }
            _ => Err(MessageReject::unsupported_message_type()),
        }
    }
}

/// Passes the session handle and inbound messages on to `main`.
struct Client {
    logons: mpsc::UnboundedSender<SessionHandle>,
    messages: mpsc::UnboundedSender<Message>,
}

impl Application for Client {
    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.logons.send(session.clone());
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.messages.send(msg.clone());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The engine reports through `tracing`: without a subscriber, its logs go nowhere.
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?.to_string();
    let config = SessionConfig::new("FIX.4.4", "SERVER");
    let server = Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(OrderDesk))?;
    tokio::spawn(server.clone().serve(listener));

    let (logons, mut logged_on) = mpsc::unbounded_channel();
    let (messages, mut received) = mpsc::unbounded_channel();
    let config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER").with_endpoint(addr);
    let app = Arc::new(Client { logons, messages });
    let client = Initiator::new(config, Arc::new(MemoryStorage::new()), app)?;
    tokio::spawn(client.clone().run());

    let session = logged_on.recv().await.ok_or("never logged on")?;
    let mut order = NewOrderSingle::new(NewOrderSingleRequired {
        cl_ord_id: "order-1".into(),
        side: Side::Buy.into(),
        transact_time: UtcTimestamp::now(),
        ord_type: OrdType::Market.into(),
    });
    order.symbol = Some("ACME".into());
    order.order_qty = Some(Decimal::from(100));
    let seq = session.send(order)?.await?;
    println!("sent order-1 as MsgSeqNum {seq}");

    let ack: ExecutionReport = received.recv().await.ok_or("no acknowledgement")?.parse()?;
    println!("acknowledged {}", ack.cl_ord_id.as_deref().unwrap_or("an order"));

    // Log out, and wait for each side's connections to close.
    client.shutdown(None).await;
    server.shutdown(None).await;
    Ok(())
}

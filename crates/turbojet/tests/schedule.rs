//! Session schedules over real connections, driven by a manual clock.

use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::{
    Acceptor, Application, Clock, Disconnect, HolidayCalendar, Initiator, InitiatorConfig, MemoryStorage,
    SessionConfig, SessionHandle, SessionId, SessionSchedule,
};

#[derive(Clone)]
struct ManualClock(Arc<Mutex<DateTime<Utc>>>);

impl ManualClock {
    fn at(time: &str) -> Self {
        Self(Arc::new(Mutex::new(utc(time))))
    }
    fn set(&self, time: &str) {
        *self.0.lock().unwrap() = utc(time);
    }
    fn clock(&self) -> Clock {
        let now = self.0.clone();
        Clock::from_fn(move || *now.lock().unwrap())
    }
}

fn utc(time: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap().and_utc()
}

#[derive(Debug, PartialEq)]
enum Event {
    LoggedOn,
    LoggedOut,
}

struct Recorder(mpsc::UnboundedSender<Event>);

impl Application for Recorder {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.0.send(Event::LoggedOn);
    }
    fn on_logout(&self, _session: &SessionId, _ended: Disconnect) {
        let _ = self.0.send(Event::LoggedOut);
    }
}

fn recorder() -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    let (tx, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder(tx)), rx)
}

async fn start_acceptor(config: SessionConfig) -> (String, mpsc::UnboundedReceiver<Event>) {
    let (app, events) = recorder();
    let acceptor = Acceptor::new(config, Arc::new(MemoryStorage::new()), app);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));
    (addr, events)
}

async fn next(rx: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    tokio::time::timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out").expect("closed")
}

// 2026-10-03 is a Saturday; 2026-10-05 a Monday.

#[tokio::test]
async fn initiator_waits_for_its_session_to_start() {
    // The client's messages carry its manual clock's fixed dates, and its counterparty's the real
    // time: neither side checks SendingTime.
    let mut server = SessionConfig::new("FIX.4.2", "SERVER");
    server.max_latency = None;
    let (addr, mut server) = start_acceptor(server).await;
    let clock = ManualClock::at("2026-10-03 10:00:00");
    let mut session = SessionConfig::new("FIX.4.2", "CLIENT");
    session.schedule = Some("daily 08:00-17:00 mon-fri".parse().unwrap());
    session.clock = clock.clock();
    session.max_latency = None;
    let (app, mut client) = recorder();
    let mut config = InitiatorConfig::new(session, "SERVER");
    config.reset_on_logon = true;
    let initiator = Initiator::new(addr, config, Arc::new(MemoryStorage::new()), app);

    let err = initiator.connect_once().await.unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotConnected);
    assert!(err.to_string().contains("next session starts 2026-10-05 08:00:00"), "{err}");

    tokio::spawn(initiator.run());
    tokio::time::sleep(Duration::from_millis(1500)).await;
    assert!(server.try_recv().is_err(), "must not connect on Saturday");

    clock.set("2026-10-05 08:00:01");
    assert_eq!(next(&mut client).await, Event::LoggedOn);
    assert_eq!(next(&mut server).await, Event::LoggedOn);
}

#[tokio::test]
async fn acceptor_logs_the_client_out_when_the_period_ends() {
    let clock = ManualClock::at("2026-10-05 16:59:00");
    let mut config = SessionConfig::new("FIX.4.2", "SERVER");
    config.schedule = Some("daily 08:00-17:00".parse().unwrap());
    config.clock = clock.clock();
    // The server's messages carry its manual clock's fixed dates, and the client's the real time:
    // neither side checks SendingTime.
    config.max_latency = None;
    let (addr, mut server) = start_acceptor(config).await;

    let (app, mut client) = recorder();
    let mut client_config = SessionConfig::new("FIX.4.2", "CLIENT");
    client_config.max_latency = None;
    let initiator =
        Initiator::new(addr, InitiatorConfig::new(client_config, "SERVER"), Arc::new(MemoryStorage::new()), app);
    let handle = initiator.handle();
    let connection = tokio::spawn(async move { initiator.connect_once().await });
    assert_eq!(next(&mut client).await, Event::LoggedOn);
    assert_eq!(next(&mut server).await, Event::LoggedOn);

    clock.set("2026-10-05 17:00:00");
    assert_eq!(next(&mut client).await, Event::LoggedOut);
    assert_eq!(next(&mut server).await, Event::LoggedOut);
    tokio::time::timeout(Duration::from_secs(5), connection).await.unwrap().unwrap().unwrap();
    assert!(!handle.is_connected());
}

// 2026-12-25 is a Friday.

#[tokio::test]
async fn initiator_waits_out_a_holiday() {
    let mut server = SessionConfig::new("FIX.4.2", "SERVER");
    server.max_latency = None;
    let (addr, mut server) = start_acceptor(server).await;
    let clock = ManualClock::at("2026-12-25 10:00:00");
    let mut session = SessionConfig::new("FIX.4.2", "CLIENT");
    let holidays: HolidayCalendar = "2026-12-25".parse().unwrap();
    session.schedule = Some("daily 08:00-17:00 mon-fri".parse::<SessionSchedule>().unwrap().with_holidays(holidays));
    session.clock = clock.clock();
    session.max_latency = None;
    let (app, mut client) = recorder();
    let mut config = InitiatorConfig::new(session, "SERVER");
    config.reset_on_logon = true;
    let initiator = Initiator::new(addr, config, Arc::new(MemoryStorage::new()), app);

    let err = initiator.connect_once().await.unwrap_err();
    assert!(err.to_string().contains("2026-12-25 is a holiday"), "{err}");
    assert!(err.to_string().contains("next session starts 2026-12-28 08:00:00"), "{err}");

    tokio::spawn(initiator.run());
    tokio::time::sleep(Duration::from_millis(1500)).await;
    assert!(server.try_recv().is_err(), "must not connect on the holiday");
    clock.set("2026-12-28 08:00:01");
    assert_eq!(next(&mut client).await, Event::LoggedOn);
    assert_eq!(next(&mut server).await, Event::LoggedOn);
}

#[tokio::test]
async fn acceptor_refuses_logons_on_a_holiday() {
    let clock = ManualClock::at("2026-12-25 10:00:00");
    let mut config = SessionConfig::new("FIX.4.2", "SERVER");
    let holidays: HolidayCalendar = "2026-12-25".parse().unwrap();
    config.schedule = Some("daily 08:00-17:00".parse::<SessionSchedule>().unwrap().with_holidays(holidays));
    config.clock = clock.clock();
    config.max_latency = None;
    let (addr, mut server) = start_acceptor(config).await;

    let (app, mut client) = recorder();
    let mut client_config = SessionConfig::new("FIX.4.2", "CLIENT");
    client_config.max_latency = None;
    let initiator =
        Initiator::new(addr, InitiatorConfig::new(client_config, "SERVER"), Arc::new(MemoryStorage::new()), app);
    // The acceptor closes the connection before logon, which connect_once reports as a failure.
    let result =
        tokio::time::timeout(Duration::from_secs(5), initiator.connect_once()).await.expect("refused promptly");
    assert!(result.is_err(), "the logon must be refused");
    assert!(client.try_recv().is_err(), "no logon on the holiday");
    assert!(server.try_recv().is_err(), "no logon on the holiday");
}

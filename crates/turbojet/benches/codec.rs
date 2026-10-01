//! Message-layer costs: wire decode/encode and typed parse/build.

mod common;

use std::hint::black_box;

use chrono::Utc;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use turbojet::Message;
use turbojet::codec::{Decoded, DecodedInto, decode, decode_into, encode};
use turbojet::fields::{FromFix, ToFix, UtcTimestamp};
use turbojet::message::{DataFields, FixMessage, utc_timestamp};
use turbojet_fix42::{ExecutionReport, NewOrderSingle, NewOrderSingleRef};

fn codec(c: &mut Criterion) {
    let order = common::with_header("CLIENT", "GATEWAY", 42, common::new_order_single(1).into());
    let order_wire = encode(&order).unwrap();
    let report = common::with_header("GATEWAY", "CLIENT", 42, common::ack_of(1).into());
    let report_wire = encode(&report).unwrap();

    let mut group = c.benchmark_group("codec");
    group.throughput(Throughput::Bytes(order_wire.len() as u64));
    group.bench_function("decode NewOrderSingle", |b| {
        b.iter(|| match decode(black_box(&order_wire)) {
            Decoded::Message(msg, len) => (msg, len),
            other => panic!("{other:?}"),
        })
    });
    group.bench_function("decode NewOrderSingle into a reused message", |b| {
        let data = DataFields::standard();
        let mut msg = Message::default();
        b.iter(|| match decode_into(black_box(&order_wire), &data, &mut msg) {
            DecodedInto::Message(len) => black_box(len),
            _ => panic!("bad order"),
        })
    });
    group.throughput(Throughput::Bytes(report_wire.len() as u64));
    group.bench_function("encode ExecutionReport", |b| b.iter(|| encode(black_box(&report)).unwrap()));
    group.finish();

    let typed_report = common::ack_of(1);
    let mut group = c.benchmark_group("typed");
    group.bench_function("parse NewOrderSingle", |b| b.iter(|| black_box(&order).parse::<NewOrderSingle>().unwrap()));
    group.bench_function("parse NewOrderSingleRef", |b| {
        b.iter(|| black_box(&order).parse::<NewOrderSingleRef>().unwrap())
    });
    group.bench_function("build ExecutionReport", |b| b.iter(|| black_box(&typed_report).to_message()));
    // With a three-entry NoAllocs group.
    let with_allocs: Message = {
        let alloc = |account: &str, shares: i64| {
            let mut alloc = turbojet_fix42::PreAllocGrp::new(account);
            alloc.alloc_shares = Some(turbojet::fields::Decimal::new(shares, 0));
            alloc
        };
        let mut order = common::new_order_single(1);
        order.allocs = vec![alloc("ACCT-A", 50), alloc("ACCT-B", 30), alloc("ACCT-C", 20)];
        common::with_header("CLIENT", "GATEWAY", 42, order.into())
    };
    group.bench_function("parse NewOrderSingle with 3 allocs", |b| {
        b.iter(|| black_box(&with_allocs).parse::<NewOrderSingle>().unwrap())
    });
    group.bench_function("parse NewOrderSingleRef with 3 allocs", |b| {
        b.iter(|| black_box(&with_allocs).parse::<NewOrderSingleRef>().unwrap())
    });
    group.bench_function("parse ExecutionReport", |b| {
        let msg: Message = typed_report.to_message();
        b.iter(|| black_box(&msg).parse::<ExecutionReport>().unwrap())
    });
    group.finish();

    // Dictionary validation, as the session does it before delivering a message.
    #[cfg(feature = "validation")]
    {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dictionaries/orchestra/OrchestraFIX42.xml");
        let validator = turbojet::validation::Validator::new(&turbojet_dictionary::Dictionary::load(path).unwrap());
        let mut group = c.benchmark_group("validation");
        group.bench_function("validate NewOrderSingle", |b| b.iter(|| validator.validate(black_box(&order)).unwrap()));
        group.bench_function("validate NewOrderSingle with 3 allocs", |b| {
            b.iter(|| validator.validate(black_box(&with_allocs)).unwrap())
        });
        group.finish();
    }

    eprintln!("wire sizes: NewOrderSingle {} bytes, ExecutionReport {} bytes", order_wire.len(), report_wire.len());
}

/// A FIX 4.4 NewOrderSingle with nested groups: 3 Parties (NoPartyIDs 453) of 2 PtysSubGrp
/// (NoPartySubIDs 802) each, with a standard header.
fn fix44_nested_order() -> Message {
    use turbojet::fields::Decimal;
    use turbojet::message::tags;
    use turbojet_fix44::{
        NewOrderSingle, OrdType, Parties, PartyIDSource, PartyRole, PartySubIDType, PtysSubGrp, Side,
    };
    let party = |id: &str, role: PartyRole| {
        let mut party = Parties::new(id);
        party.party_id_source = Some(PartyIDSource::Proprietary);
        party.party_role = Some(role);
        party.party_sub_ids = [("DESK-1", PartySubIDType::Application), ("TRADER-7", PartySubIDType::Person)]
            .into_iter()
            .map(|(sub_id, kind)| {
                let mut sub = PtysSubGrp::new(sub_id);
                sub.party_sub_id_type = Some(kind);
                sub
            })
            .collect();
        party
    };
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, UtcTimestamp::now(), OrdType::Limit);
    order.party_ids = vec![
        party("FIRM-A", PartyRole::ExecutingFirm),
        party("CLIENT-B", PartyRole::ClientID),
        party("CLEAR-C", PartyRole::ClearingFirm),
    ];
    order.account = Some("ACCT-001".into());
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(Decimal::new(100, 0));
    order.price = Some(Decimal::new(15025, 2));
    let body: Message = order.into();
    let mut msg = Message::default();
    msg.push(tags::BEGIN_STRING, "FIX.4.4");
    msg.push(tags::MSG_TYPE, body.msg_type());
    msg.push(tags::SENDER_COMP_ID, "CLIENT");
    msg.push(tags::TARGET_COMP_ID, "GATEWAY");
    msg.push(tags::MSG_SEQ_NUM, 42u64);
    msg.push(tags::SENDING_TIME, utc_timestamp());
    for (tag, value) in body.fields() {
        if tag != tags::MSG_TYPE {
            msg.push(tag, value);
        }
    }
    msg
}

/// Parsing nested groups. Owned parsing checks each entry and then builds it, so nested entries
/// are parsed more than once; borrowed groups parse their entries again as they are iterated.
fn nested_groups(c: &mut Criterion) {
    let nested = fix44_nested_order();
    let mut group = c.benchmark_group("typed");
    group.bench_function("parse FIX 4.4 NewOrderSingle with nested groups", |b| {
        b.iter(|| black_box(&nested).parse::<turbojet_fix44::NewOrderSingle>().unwrap())
    });
    group.bench_function("parse FIX 4.4 NewOrderSingleRef with nested groups", |b| {
        b.iter(|| black_box(&nested).parse::<turbojet_fix44::NewOrderSingleRef>().unwrap())
    });
    // Borrowed groups parse their entries as they are iterated: read them all.
    group.bench_function("parse FIX 4.4 NewOrderSingleRef with nested groups, read every entry", |b| {
        b.iter(|| {
            let order = black_box(&nested).parse::<turbojet_fix44::NewOrderSingleRef>().unwrap();
            for party in &order.party_ids {
                black_box(party.party_id);
                black_box(party.party_id_source);
                black_box(party.party_role);
                for sub in &party.party_sub_ids {
                    black_box(sub.party_sub_id);
                    black_box(sub.party_sub_id_type);
                }
            }
        })
    });
    group.finish();

    eprintln!("wire size: FIX 4.4 NewOrderSingle with nested groups {} bytes", encode(&nested).unwrap().len());
    eprintln!(
        "type sizes: NewOrderSingle {} bytes, NewOrderSingleRef {} bytes, \
         FIX 4.4 NewOrderSingle {} bytes, FIX 4.4 NewOrderSingleRef {} bytes",
        size_of::<NewOrderSingle>(),
        size_of::<NewOrderSingleRef>(),
        size_of::<turbojet_fix44::NewOrderSingle>(),
        size_of::<turbojet_fix44::NewOrderSingleRef>(),
    );
}

fn timestamps(c: &mut Criterion) {
    let mut group = c.benchmark_group("timestamp");
    // A fixed instant: formatting the same second repeatedly, as a busy session does.
    let instant = UtcTimestamp::from_fix("20260927-03:20:48.544").unwrap();
    let mut out = String::with_capacity(32);
    group.bench_function("write UtcTimestamp", |b| {
        b.iter(|| {
            out.clear();
            black_box(&instant).write_fix(&mut out);
        })
    });
    let micros = UtcTimestamp::from_fix("20260927-03:20:48.544123").unwrap();
    group.bench_function("write UtcTimestamp in microseconds", |b| {
        b.iter(|| {
            out.clear();
            black_box(&micros).write_fix(&mut out);
        })
    });
    group.bench_function("parse UtcTimestamp", |b| {
        b.iter(|| UtcTimestamp::from_fix(black_box("20260927-03:20:48.544")))
    });
    // A new second every call: the worst case for any per-second caching.
    let mut seconds = 0i64;
    group.bench_function("write UtcTimestamp, new second each time", |b| {
        b.iter(|| {
            seconds += 1;
            out.clear();
            (instant + chrono::Duration::seconds(seconds)).write_fix(&mut out);
        })
    });
    group.bench_function("utc_timestamp() (now, as a String)", |b| b.iter(utc_timestamp));
    group.bench_function("Utc::now() alone", |b| b.iter(Utc::now));
    group.finish();
}

criterion_group!(benches, codec, nested_groups, timestamps);
criterion_main!(benches);

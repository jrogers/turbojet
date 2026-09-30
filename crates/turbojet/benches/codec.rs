//! Message-layer costs: wire decode/encode and typed parse/build.

mod common;

use std::hint::black_box;

use chrono::Utc;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use turbojet::Message;
use turbojet::codec::{Decoded, DecodedInto, decode, decode_into, encode};
use turbojet::fields::{FromFix, ToFix, UtcTimestamp};
use turbojet::message::{DataFields, FixMessage, utc_timestamp};
use turbojet_fix42::{ExecutionReport, NewOrderSingle};

fn codec(c: &mut Criterion) {
    let order = common::with_header("CLIENT", "GATEWAY", 42, common::new_order_single(1).into());
    let order_wire = encode(&order).unwrap();
    let report = common::with_header("GATEWAY", "CLIENT", 42, common::ack(common::new_order_single(1), 1).into());
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

    let typed_report = common::ack(common::new_order_single(1), 1);
    let mut group = c.benchmark_group("typed");
    group.bench_function("parse NewOrderSingle", |b| b.iter(|| black_box(&order).parse::<NewOrderSingle>().unwrap()));
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

criterion_group!(benches, codec, timestamps);
criterion_main!(benches);

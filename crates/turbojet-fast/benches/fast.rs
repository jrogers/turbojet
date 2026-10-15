//! FAST decoding and encoding of an incremental refresh, by number of entries: a message whose
//! values follow the previous one's (copies and small deltas, as a feed sends most), and one
//! decoded from a reset decoder, every value explicit.

mod common;

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use turbojet::Message;
use turbojet_fast::{Decoder, Encoder};

fn decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast");
    for entries in [1, 10, 50] {
        let (first, second) = common::encoded(entries);
        let mut decoder = Decoder::new(common::templates());
        let mut msg = Message::default();
        decoder.decode_into(&first, &mut msg).unwrap();
        eprintln!("refresh of {entries}: {} bytes first, {} bytes after", first.len(), second.len());

        group.throughput(Throughput::Bytes(second.len() as u64));
        // The previous values settle: each decode of the same message copies what the last left.
        group.bench_function(format!("decode refresh of {entries}, following the previous"), |b| {
            b.iter(|| decoder.decode_into(black_box(&second), &mut msg).unwrap())
        });
        group.throughput(Throughput::Bytes(first.len() as u64));
        group.bench_function(format!("decode refresh of {entries}, after a reset"), |b| {
            b.iter(|| {
                decoder.reset();
                decoder.decode_into(black_box(&first), &mut msg).unwrap()
            })
        });

        let refresh = common::refresh(2, entries);
        let mut encoder = Encoder::new(common::templates());
        let mut out = Vec::new();
        encoder.encode(1, &common::refresh(1, entries), &mut out).unwrap();
        group.bench_function(format!("encode refresh of {entries}, following the previous"), |b| {
            b.iter(|| {
                out.clear();
                encoder.encode(1, black_box(&refresh), &mut out).unwrap()
            })
        });
    }
    group.finish();
}

criterion_group!(benches, decode);
criterion_main!(benches);

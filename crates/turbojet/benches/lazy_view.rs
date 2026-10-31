//! A generated message's view against its borrowed form: FIX 4.2's NewOrderSingle (68 fields), on
//! the benchmarks' order (10 body fields set), and its ExecutionReport, on the benchmarks' ack of
//! that order (17 body fields set). The borrowed form converts every field present as
//! it parses; the view only finds them, and converts each one read.

mod common;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use turbojet::message::FieldError;
use turbojet_fix::fix42::*;

/// What an order desk reads of an order: its ten fields, as the acknowledgement in
/// `common::ack` and a risk check would.
fn read_ref(order: &NewOrderSingleRef<'_>) -> usize {
    let mut n = order.cl_ord_id.len() + order.symbol.len();
    n += usize::from(order.side.known().is_some()) + usize::from(order.ord_type.known().is_some());
    n += usize::from(order.handl_inst.known().is_some()) + order.account.map_or(0, str::len);
    n += usize::from(order.order_qty.is_some()) + usize::from(order.price.is_some());
    n += usize::from(order.time_in_force.is_some()) + usize::from(order.transact_time.datetime().timestamp() > 0);
    n
}

fn read_view(order: &NewOrderSingleView<'_>) -> Result<usize, FieldError> {
    let mut n = order.cl_ord_id()?.len() + order.symbol()?.len();
    n += usize::from(order.side()?.known().is_some()) + usize::from(order.ord_type()?.known().is_some());
    n += usize::from(order.handl_inst()?.known().is_some()) + order.account()?.map_or(0, str::len);
    n += usize::from(order.order_qty()?.is_some()) + usize::from(order.price()?.is_some());
    n += usize::from(order.time_in_force()?.is_some()) + usize::from(order.transact_time()?.datetime().timestamp() > 0);
    Ok(n)
}

/// What an order desk reads of an execution report: what it says happened to which order.
fn read_report_ref(report: &ExecutionReportRef<'_>) -> usize {
    let mut n = report.order_id.len() + report.exec_id.len() + report.cl_ord_id.map_or(0, str::len);
    n += usize::from(report.exec_type.known().is_some()) + usize::from(report.ord_status.known().is_some());
    n += report.symbol.len() + usize::from(report.side.known().is_some());
    n += usize::from(report.leaves_qty.is_zero()) + usize::from(report.cum_qty.is_zero());
    n += usize::from(report.avg_px.is_zero());
    n
}

fn read_report_view(report: &ExecutionReportView<'_>) -> Result<usize, FieldError> {
    let mut n = report.order_id()?.len() + report.exec_id()?.len() + report.cl_ord_id()?.map_or(0, str::len);
    n += usize::from(report.exec_type()?.known().is_some()) + usize::from(report.ord_status()?.known().is_some());
    n += report.symbol()?.len() + usize::from(report.side()?.known().is_some());
    n += usize::from(report.leaves_qty()?.is_zero()) + usize::from(report.cum_qty()?.is_zero());
    n += usize::from(report.avg_px()?.is_zero());
    Ok(n)
}

fn lazy_view(c: &mut Criterion) {
    let order = common::with_header("CLIENT", "GATEWAY", 42, common::new_order_single(1).into());
    let parsed: NewOrderSingleRef = order.parse().unwrap();
    let view: NewOrderSingleView = order.parse().unwrap();
    assert_eq!(read_ref(&parsed), read_view(&view).unwrap(), "the view reads what the struct holds");
    eprintln!(
        "sizes: NewOrderSingleRef {} bytes, NewOrderSingleView {} bytes",
        size_of::<NewOrderSingleRef>(),
        size_of::<NewOrderSingleView>()
    );

    let mut group = c.benchmark_group("lazy view");
    group.bench_function("Ref: parse", |b| b.iter(|| black_box(&order).parse::<NewOrderSingleRef>().unwrap()));
    group.bench_function("Ref: parse, read 2 fields", |b| {
        b.iter(|| {
            let order = black_box(&order).parse::<NewOrderSingleRef>().unwrap();
            order.cl_ord_id.len() + order.symbol.len()
        })
    });
    group.bench_function("Ref: parse, read 10 fields", |b| {
        b.iter(|| read_ref(&black_box(&order).parse::<NewOrderSingleRef>().unwrap()))
    });
    group.bench_function("View: parse", |b| b.iter(|| black_box(&order).parse::<NewOrderSingleView>().unwrap()));
    group.bench_function("View: parse, read 2 fields", |b| {
        b.iter(|| {
            let order = black_box(&order).parse::<NewOrderSingleView>().unwrap();
            order.cl_ord_id().unwrap().len() + order.symbol().unwrap().len()
        })
    });
    group.bench_function("View: parse, read 10 fields", |b| {
        b.iter(|| read_view(&black_box(&order).parse::<NewOrderSingleView>().unwrap()).unwrap())
    });
    group.finish();

    let report = common::with_header("GATEWAY", "CLIENT", 42, common::ack_of(1).into());
    let parsed: ExecutionReportRef = report.parse().unwrap();
    let view: ExecutionReportView = report.parse().unwrap();
    assert_eq!(read_report_ref(&parsed), read_report_view(&view).unwrap(), "the view reads what the struct holds");
    eprintln!(
        "sizes: ExecutionReportRef {} bytes, ExecutionReportView {} bytes",
        size_of::<ExecutionReportRef>(),
        size_of::<ExecutionReportView>()
    );

    let mut group = c.benchmark_group("lazy view, execution report");
    group.bench_function("Ref: parse", |b| b.iter(|| black_box(&report).parse::<ExecutionReportRef>().unwrap()));
    group.bench_function("Ref: parse, read 2 fields", |b| {
        b.iter(|| {
            let report = black_box(&report).parse::<ExecutionReportRef>().unwrap();
            report.cl_ord_id.map_or(0, str::len) + usize::from(report.ord_status.known().is_some())
        })
    });
    group.bench_function("Ref: parse, read 10 fields", |b| {
        b.iter(|| read_report_ref(&black_box(&report).parse::<ExecutionReportRef>().unwrap()))
    });
    group.bench_function("View: parse", |b| b.iter(|| black_box(&report).parse::<ExecutionReportView>().unwrap()));
    group.bench_function("View: parse, read 2 fields", |b| {
        b.iter(|| {
            let report = black_box(&report).parse::<ExecutionReportView>().unwrap();
            report.cl_ord_id().unwrap().map_or(0, str::len)
                + usize::from(report.ord_status().unwrap().known().is_some())
        })
    });
    group.bench_function("View: parse, read 10 fields", |b| {
        b.iter(|| read_report_view(&black_box(&report).parse::<ExecutionReportView>().unwrap()).unwrap())
    });
    group.finish();
}

criterion_group!(benches, lazy_view);
criterion_main!(benches);

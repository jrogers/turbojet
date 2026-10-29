//! A prototype of ROADMAP §4 "Large generated messages", option 3: a borrowed view that keeps the
//! message and a table from each declared field to its first occurrence, and converts a field only
//! when it's read. Measured against the generated borrowed form of the same FIX 4.2
//! NewOrderSingle (68 fields, 1,184 bytes), on the benchmarks' order (10 body fields set).
//!
//! Two variants: `lazy` only records where each field is and checks the required ones are
//! present, so a malformed value is reported when it's read; `checked` also converts every field
//! present while indexing, so errors come from the parse as they do now, but keeps nothing.
//!
//! Prototype limits: groups are recorded by their NumInGroup field only (their entries' tags
//! aren't skipped, which this message's top-level tags don't need), data fields are found again
//! when read, and the table is `u16`, so a message may have at most 65,534 fields.

mod common;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use turbojet::Message;
use turbojet::fields::{Code, CompactString, Decimal, FieldRef, MonthYear, NaiveDate, UtcTimestamp, ValueError};
use turbojet::message::{FieldError, FieldErrorKind, FieldView};
use turbojet_fix::fix42::tags::*;
use turbojet_fix::fix42::*;

/// No occurrence.
const ABSENT: u16 = u16::MAX;

fn conversion_error(tag: u32, raw: &str, error: ValueError) -> FieldError {
    let kind = match error {
        ValueError::Incorrect => FieldErrorKind::IncorrectValue(raw.to_string()),
        _ => FieldErrorKind::IncorrectFormat(raw.to_string()),
    };
    FieldError { tag, kind }
}

/// Converts the field at `index` of `view`, if it's there.
fn read<'a, T: FieldRef<'a>>(view: &FieldView<'a>, index: u16, tag: u32) -> Result<Option<T::Ref>, FieldError> {
    if index == ABSENT {
        return Ok(None);
    }
    match view.at(usize::from(index)).1 {
        Some(raw) => T::parse_ref(raw).map(Some).map_err(|e| conversion_error(tag, raw, e)),
        None => Err(conversion_error(tag, "", ValueError::Format)),
    }
}

macro_rules! lazy_view {
    ($name:ident = $msg_type:literal { $( $field:ident : $presence:ident $ty:ty = $tag:path ),+ $(,)? }) => {
        #[allow(non_camel_case_types, dead_code)]
        #[repr(usize)]
        enum Slot { $( $field ),+ }

        const COUNT: usize = [$( Slot::$field ),+].len();

        /// The message, and where each declared field first occurs in it.
        #[derive(Debug, Clone)]
        struct $name<'a> {
            view: FieldView<'a>,
            at: [u16; COUNT],
        }

        #[allow(dead_code)]
        impl<'a> $name<'a> {
            /// Indexes the fields; with `check`, converts each one present too, failing on the first
            /// that doesn't convert.
            fn parse(msg: &'a Message, check: bool) -> Result<Self, FieldError> {
                if msg.get(MSG_TYPE) != Some($msg_type) {
                    return Err(FieldError { tag: MSG_TYPE, kind: FieldErrorKind::IncorrectValue(msg.get(MSG_TYPE).unwrap_or_default().into()) });
                }
                let view = msg.view();
                let mut at = [ABSENT; COUNT];
                let (mut index, end) = view.bounds();
                while index < end {
                    index = view.skip_hidden(index);
                    if index >= end {
                        break;
                    }
                    let (tag, raw) = view.at(index);
                    // A match, as the generated parser's is, so the compiler can dispatch on the tag.
                    match tag {
                        $( $tag => {
                            let slot = &mut at[Slot::$field as usize];
                            if *slot == ABSENT {
                                *slot = u16::try_from(index).expect("prototype: under 65,535 fields");
                                if check {
                                    lazy_view!(@check $presence $ty, raw, $tag);
                                }
                            }
                        } )+
                        _ => {}
                    }
                    index += 1;
                }
                let parsed = Self { view, at };
                $( lazy_view!(@required $presence, parsed, $field, $tag); )+
                Ok(parsed)
            }

            $( lazy_view!(@accessor $presence $ty, $field, $tag); )+
        }
    };
    (@required req, $parsed:ident, $field:ident, $tag:expr) => {
        if $parsed.at[Slot::$field as usize] == ABSENT {
            return Err(FieldError { tag: $tag, kind: FieldErrorKind::Missing });
        }
    };
    (@required $other:ident, $parsed:ident, $field:ident, $tag:expr) => {};
    // Converts a field as the pass finds it, keeping nothing.
    (@check req $ty:ty, $raw:ident, $tag:expr) => { lazy_view!(@check opt $ty, $raw, $tag) };
    (@check opt $ty:ty, $raw:ident, $tag:expr) => {
        match $raw {
            Some(raw) => {
                if let Err(e) = <$ty as FieldRef<'a>>::parse_ref(raw) {
                    return Err(conversion_error($tag, raw, e));
                }
            }
            None => return Err(conversion_error($tag, "", ValueError::Format)),
        }
    };
    (@check $other:ident $ty:ty, $raw:ident, $tag:expr) => {};
    (@accessor req $ty:ty, $field:ident, $tag:expr) => {
        fn $field(&self) -> Result<<$ty as FieldRef<'a>>::Ref, FieldError> {
            Ok(read::<$ty>(&self.view, self.at[Slot::$field as usize], $tag)?.expect("checked present when parsed"))
        }
    };
    (@accessor opt $ty:ty, $field:ident, $tag:expr) => {
        fn $field(&self) -> Result<Option<<$ty as FieldRef<'a>>::Ref>, FieldError> {
            read::<$ty>(&self.view, self.at[Slot::$field as usize], $tag)
        }
    };
    (@accessor group $ty:ty, $field:ident, $tag:expr) => {
        fn $field(&self) -> Result<Vec<FieldView<'a>>, FieldError> {
            self.view.clone().group($tag, &<$ty as turbojet::message::FixGroup>::SPEC)
        }
    };
    (@accessor opt_data $ty:ty, $field:ident, $tag:expr) => {
        fn $field(&self) -> Option<&'a [u8]> {
            self.view.get_bytes($tag)
        }
    };
}

lazy_view! {
    NewOrderSingleView = "D" {
        cl_ord_id: req CompactString = CL_ORD_ID,
        client_id: opt CompactString = CLIENT_ID,
        exec_broker: opt CompactString = EXEC_BROKER,
        account: opt CompactString = ACCOUNT,
        allocs: group PreAllocGrp = NO_ALLOCS,
        settlmnt_typ: opt Code<SettlmntTyp> = SETTLMNT_TYP,
        fut_sett_date: opt NaiveDate = FUT_SETT_DATE,
        handl_inst: req Code<HandlInst> = HANDL_INST,
        exec_inst: opt Vec<Code<ExecInst>> = EXEC_INST,
        min_qty: opt Decimal = MIN_QTY,
        max_floor: opt Decimal = MAX_FLOOR,
        ex_destination: opt CompactString = EX_DESTINATION,
        trading_sessions: group TrdgSesGrp = NO_TRADING_SESSIONS,
        process_code: opt Code<ProcessCode> = PROCESS_CODE,
        symbol: req CompactString = SYMBOL,
        symbol_sfx: opt CompactString = SYMBOL_SFX,
        security_id: opt CompactString = SECURITY_ID,
        id_source: opt Code<IDSource> = ID_SOURCE,
        security_type: opt Code<SecurityType> = SECURITY_TYPE,
        maturity_month_year: opt MonthYear = MATURITY_MONTH_YEAR,
        maturity_day: opt i64 = MATURITY_DAY,
        put_or_call: opt Code<PutOrCall> = PUT_OR_CALL,
        strike_price: opt Decimal = STRIKE_PRICE,
        opt_attribute: opt char = OPT_ATTRIBUTE,
        contract_multiplier: opt Decimal = CONTRACT_MULTIPLIER,
        coupon_rate: opt Decimal = COUPON_RATE,
        security_exchange: opt CompactString = SECURITY_EXCHANGE,
        issuer: opt CompactString = ISSUER,
        encoded_issuer: opt_data Vec<u8> = ENCODED_ISSUER,
        security_desc: opt CompactString = SECURITY_DESC,
        encoded_security_desc: opt_data Vec<u8> = ENCODED_SECURITY_DESC,
        prev_close_px: opt Decimal = PREV_CLOSE_PX,
        side: req Code<Side> = SIDE,
        locate_reqd: opt bool = LOCATE_REQD,
        transact_time: req UtcTimestamp = TRANSACT_TIME,
        order_qty: opt Decimal = ORDER_QTY,
        cash_order_qty: opt Decimal = CASH_ORDER_QTY,
        ord_type: req Code<OrdType> = ORD_TYPE,
        price: opt Decimal = PRICE,
        stop_px: opt Decimal = STOP_PX,
        currency: opt CompactString = CURRENCY,
        compliance_id: opt CompactString = COMPLIANCE_ID,
        solicited_flag: opt bool = SOLICITED_FLAG,
        ioi_id: opt CompactString = IOI_ID,
        quote_id: opt CompactString = QUOTE_ID,
        time_in_force: opt Code<TimeInForce> = TIME_IN_FORCE,
        effective_time: opt UtcTimestamp = EFFECTIVE_TIME,
        expire_date: opt NaiveDate = EXPIRE_DATE,
        expire_time: opt UtcTimestamp = EXPIRE_TIME,
        gt_booking_inst: opt Code<GTBookingInst> = GT_BOOKING_INST,
        commission: opt Decimal = COMMISSION,
        comm_type: opt Code<CommType> = COMM_TYPE,
        rule80_a: opt Code<Rule80A> = RULE80_A,
        forex_req: opt bool = FOREX_REQ,
        settl_currency: opt CompactString = SETTL_CURRENCY,
        text: opt CompactString = TEXT,
        encoded_text: opt_data Vec<u8> = ENCODED_TEXT,
        fut_sett_date2: opt NaiveDate = FUT_SETT_DATE2,
        order_qty2: opt Decimal = ORDER_QTY2,
        open_close: opt Code<OpenClose> = OPEN_CLOSE,
        covered_or_uncovered: opt Code<CoveredOrUncovered> = COVERED_OR_UNCOVERED,
        customer_or_firm: opt Code<CustomerOrFirm> = CUSTOMER_OR_FIRM,
        max_show: opt Decimal = MAX_SHOW,
        peg_difference: opt Decimal = PEG_DIFFERENCE,
        discretion_inst: opt Code<DiscretionInst> = DISCRETION_INST,
        discretion_offset: opt Decimal = DISCRETION_OFFSET,
        clearing_firm: opt CompactString = CLEARING_FIRM,
        clearing_account: opt CompactString = CLEARING_ACCOUNT,
    }
}

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

fn lazy_view(c: &mut Criterion) {
    let order = common::with_header("CLIENT", "GATEWAY", 42, common::new_order_single(1).into());
    let parsed: NewOrderSingleRef = order.parse().unwrap();
    let view = NewOrderSingleView::parse(&order, true).unwrap();
    assert_eq!(read_ref(&parsed), read_view(&view).unwrap(), "the view reads what the struct holds");
    eprintln!(
        "sizes: NewOrderSingleRef {} bytes, NewOrderSingleView {} bytes",
        size_of::<NewOrderSingleRef>(),
        size_of::<NewOrderSingleView>()
    );

    let mut group = c.benchmark_group("lazy view");
    group
        .bench_function("generated Ref: parse", |b| b.iter(|| black_box(&order).parse::<NewOrderSingleRef>().unwrap()));
    group.bench_function("generated Ref: parse, read 10 fields", |b| {
        b.iter(|| read_ref(&black_box(&order).parse::<NewOrderSingleRef>().unwrap()))
    });
    group.bench_function("generated Ref: parse, read 2 fields", |b| {
        b.iter(|| {
            let order = black_box(&order).parse::<NewOrderSingleRef>().unwrap();
            order.cl_ord_id.len() + order.symbol.len()
        })
    });
    group.bench_function("lazy view: parse, read 2 fields", |b| {
        b.iter(|| {
            let order = NewOrderSingleView::parse(black_box(&order), false).unwrap();
            order.cl_ord_id().unwrap().len() + order.symbol().unwrap().len()
        })
    });
    group.bench_function("lazy view: parse", |b| {
        b.iter(|| NewOrderSingleView::parse(black_box(&order), false).unwrap())
    });
    group.bench_function("lazy view: parse, read 10 fields", |b| {
        b.iter(|| read_view(&NewOrderSingleView::parse(black_box(&order), false).unwrap()).unwrap())
    });
    group.bench_function("checked view: parse", |b| {
        b.iter(|| NewOrderSingleView::parse(black_box(&order), true).unwrap())
    });
    group.bench_function("checked view: parse, read 10 fields", |b| {
        b.iter(|| read_view(&NewOrderSingleView::parse(black_box(&order), true).unwrap()).unwrap())
    });
    group.finish();
}

criterion_group!(benches, lazy_view);
criterion_main!(benches);

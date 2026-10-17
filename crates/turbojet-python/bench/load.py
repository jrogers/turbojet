"""The engine's `load` example, from Python: one session sends NewOrderSingles at a set rate and
reports how long each took to be answered with an ExecutionReport, from when it was due (open
loop, as `load` explains), as Python sees it.

Start the gateway with `cargo run --release --example gateway --all-features -- --allow-any`,
build the bindings with `uvx maturin develop --release`, then run
`uv run python bench/load.py [--addr ADDR] [--rate R] [--duration S] [--warmup S]`.
"""

import argparse
import asyncio
import os
import tempfile
import time
from datetime import datetime, timezone

import turbojet
from turbojet import AppMessage, Logon, Message, tags


def order(cl_ord_id):
    now = datetime.now(timezone.utc).strftime("%Y%m%d-%H:%M:%S.%f")[:-3]
    return Message(
        "D",
        [
            (tags.CL_ORD_ID, cl_ord_id),
            (tags.HANDL_INST, "1"),
            (tags.SYMBOL, "AAPL"),
            (tags.SIDE, "1"),
            (tags.TRANSACT_TIME, now),
            (tags.ORDER_QTY, "100"),
            (tags.ORD_TYPE, "2"),
            (tags.PRICE, "150.25"),
        ],
    )


def percentile(ordered, quantile):
    """Nearest rank, as `load` reports them."""
    if not ordered:
        return None
    rank = max(1, -(-len(ordered) * quantile // 1))
    return ordered[int(rank) - 1]


async def run(args):
    with tempfile.TemporaryDirectory() as dir:
        path = os.path.join(dir, "sessions.toml")
        with open(path, "w") as f:
            f.write(
                f"""[initiator.load]
begin_string = "FIX.4.2"
sender_comp_id = "PYLOAD"
target_comp_id = "GATEWAY"
connect = ["{args.addr}"]
reset_on_logon = true
send_queue = 65536
"""
            )
        async with turbojet.load(path) as engine:
            events = engine.events()
            session = None
            async for event in events:
                if isinstance(event, Logon):
                    session = event.session
                    break
            warmup = int(args.warmup * args.rate)
            total = warmup + int(args.duration * args.rate)
            start = time.perf_counter_ns() + 100_000_000
            period = 1e9 / args.rate
            run_id = f"{time.time_ns()}-"
            latencies = []
            from_send = []
            sent_at = [0] * total
            answered = set()
            lag_max = 0
            unsent = 0

            async def send_all():
                nonlocal lag_max, unsent
                for k in range(total):
                    due = start + int(k * period)
                    wait = due - time.perf_counter_ns()
                    if wait > 0:
                        await asyncio.sleep(wait / 1e9)
                    if k >= warmup:
                        lag_max = max(lag_max, time.perf_counter_ns() - due)
                    try:
                        msg = order(f"{run_id}{k}")
                        sent_at[k] = time.perf_counter_ns()
                        session.send_nowait(msg)
                    except turbojet.Error:
                        unsent += 1

            sender = asyncio.create_task(send_all())

            async def receive():
                async for event in events:
                    if not isinstance(event, AppMessage):
                        continue
                    now = time.perf_counter_ns()
                    cl_ord_id = event.msg.get(tags.CL_ORD_ID) or ""
                    if event.msg.msg_type == "8" and cl_ord_id.startswith(run_id):
                        k = int(cl_ord_id[len(run_id) :])
                        if k not in answered:
                            answered.add(k)
                            if k >= warmup:
                                latencies.append(now - (start + int(k * period)))
                                from_send.append(now - sent_at[k])
                    event.done()
                    if len(answered) >= total - unsent and sender.done():
                        return

            try:
                await asyncio.wait_for(asyncio.gather(sender, receive()), args.warmup + args.duration + 10)
            except asyncio.TimeoutError:
                pass
            sent = total - unsent
            elapsed = (total * period) / 1e9
            print(f"sent {sent} orders ({unsent} not sent) in {elapsed:.2f} s: {sent / elapsed:.0f}/s")
            print(f"answered {len(answered)}; {sent - len(answered)} unanswered")
            latencies.sort()
            print(f"latency from due time, {len(latencies)} orders after {args.warmup} s of warm-up:")
            for name, quantile in [("p50", 0.5), ("p90", 0.9), ("p99", 0.99), ("p99.9", 0.999), ("p99.99", 0.9999)]:
                value = percentile(latencies, quantile)
                if value is not None:
                    print(f"  {name:<7}{value / 1000:.1f} µs")
            if latencies:
                print(f"  {'max':<7}{latencies[-1] / 1000:.1f} µs")
            print(f"sender fell behind by {lag_max / 1000:.1f} µs at worst")
            from_send.sort()
            print("latency from sending, without the sender's lateness:")
            for name, quantile in [("p50", 0.5), ("p99", 0.99), ("p99.9", 0.999)]:
                value = percentile(from_send, quantile)
                if value is not None:
                    print(f"  {name:<7}{value / 1000:.1f} µs")


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--addr", default="127.0.0.1:9876")
    parser.add_argument("--rate", type=float, default=1000)
    parser.add_argument("--duration", type=float, default=10)
    parser.add_argument("--warmup", type=float, default=2)
    asyncio.run(run(parser.parse_args()))


if __name__ == "__main__":
    main()

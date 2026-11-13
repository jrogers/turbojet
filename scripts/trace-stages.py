"""Splits each order's round trip from aws-bench.sh's trace run into its stages.

    python3 -I scripts/trace-stages.py DIR

DIR holds ack-trace.txt and load-trace.txt, the bpftrace output on each machine (`r NSECS`
when a spinning thread's read last returned bytes before a write, `w NSECS` when its write began, `p NSECS` when
`load`'s main thread called getpid), and stamps.txt, `load --stamps`: an order's number, then when
it was due, built, handed to its session and answered, in nanoseconds from its last getpid call
(after a comment line, `# getpid call I of N, between readings T ns apart`).
"""

import bisect
import sys
from pathlib import Path


def events(path):
    """The reads' and the writes' times, and both in order as (kind, time)."""
    reads, writes, both = [], [], []
    for line in path.read_text().splitlines():
        fields = line.split()
        if fields[:1] == ["r"]:
            reads.append(int(fields[1]))
            both.append(("r", int(fields[1])))
        elif fields[:1] == ["w"]:
            writes.append(int(fields[1]))
            both.append(("w", int(fields[1])))
    return reads, writes, both


def percentile(ordered, per_mille):
    """Nearest rank."""
    return ordered[max(1, -(-len(ordered) * per_mille // 1000)) - 1]


def show(name, values):
    values = sorted(values)
    if not values:
        print(f"  {name:<34} none")
        return
    cells = "  ".join(f"p{p / 10:g} {percentile(values, p) / 1000:6.1f}" for p in (500, 900, 990))
    print(f"  {name:<34} {cells} µs  ({len(values)})")


def main(directory):
    d = Path(directory)
    _, _, ack = events(d / "ack-trace.txt")
    # ack: from each read that returned bytes to the write after it.
    show("ack, order read to report written", [w - r for (a, r), (b, w) in zip(ack, ack[1:]) if (a, b) == ("r", "w")])
    reads, writes, _ = events(d / "load-trace.txt")
    # --stamps' zero: one of load's last getpid calls, which its first line names.
    calls = [int(line.split()[1]) for line in (d / "load-trace.txt").read_text().splitlines() if line.startswith("p ")]
    first = (d / "stamps.txt").read_text().split("\n", 1)[0].split()
    call, count = int(first[3]), int(first[5].rstrip(","))
    zero = calls[len(calls) - count + call]
    stages = {
        name: []
        for name in (
            "due to built",
            "built to handed over",
            "built to written",
            "written to report read (socket)",
            "report read to answered",
            "due to answered",
        )
    }
    for line in (d / "stamps.txt").read_text().splitlines():
        if line.startswith("#"):
            print(f"stamps: {line[1:].strip()}, so placed to within half that")
            continue
        _, due, built, sent, answered = (int(field) + zero for field in line.split())
        # The first write once the order was built (often before the sender's send() has
        # returned), and the first read after that.
        i = bisect.bisect_left(writes, built)
        if i == len(writes):
            continue
        j = bisect.bisect_left(reads, writes[i])
        if j == len(reads) or reads[j] > answered:
            continue
        times = (built - due, sent - built, writes[i] - built, reads[j] - writes[i], answered - reads[j], answered - due)
        for values, value in zip(stages.values(), times):
            values.append(value)
    print("load, each order:")
    for name, values in stages.items():
        show(name, values)


if __name__ == "__main__":
    main(sys.argv[1])

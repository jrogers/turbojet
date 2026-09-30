# Turbojet style

How Turbojet is written, and why. It borrows from TigerBeetle's
[Tiger Style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md), which
applies NASA's [Power of Ten](https://spinroot.com/gerard/pdf/P10.pdf) rules to a database, and
adapts it to a Rust library on tokio. Where Turbojet departs from it, the last section says so and
says why.

A FIX engine sits between a trading system and its counterparties. A bug in it can send a
duplicate order, drop a fill or lose a sequence number. So the priorities, in order, are
**safety, performance and developer experience**. Where they conflict, safety wins. Mostly they
don't: simple code tends to be easier to make correct, to make fast and to read.

## Simplicity and debt

- The first design that works is rarely the simple one. Spend the thinking up front, and revise
  until the design is simple. It costs less than fixing it once it's in use.
- Don't ship a known showstopper: work that grows without limit, a latency spike on a common path,
  or an error that's silently ignored. If a problem is accepted for now, it goes in
  [ROADMAP.md](ROADMAP.md) with its size, so the decision is written down rather than forgotten.

## Safety

### Put a limit on everything

Every queue, buffer, retry and loop over outside input has a fixed upper bound. The bound is a
named constant or a configuration field, with a comment saying why it has that value. A limit
turns an unbounded failure (memory exhaustion, a stalled task, a latency tail) into one that is
bounded and visible. Examples: `MAX_BODY_LENGTH` and `MAX_HEADER_FIELD_LEN` in `codec.rs`,
`MAX_QUEUED` for messages held ahead of a gap in `session.rs`, and `MAX_COMMANDS_PER_BATCH` in
`connection.rs`. Where something still grows without a limit, the roadmap lists it.

Recursion must have a depth that Turbojet's own types fix, never one that input controls. For
example, `scan_group` in `message.rs` recurses into nested repeating groups only as deep as the
static `GroupSpec` nests, however many levels a counterparty sends.

### Assert invariants

Assertions turn a correctness bug into a crash, which is noisy and easy to find, rather than
corrupt state. They also multiply what fuzzing finds, because the fuzzer detects a broken
invariant as soon as it happens, not when it eventually causes a visible failure.

- Assert preconditions, postconditions and invariants: what a function requires, what it
  guarantees, and what must always be true of a type.
- **Pair assertions.** Check a property on two different code paths, for example as a message is
  stored and again as it is read back, or as it is encoded and again as it is framed. A bug has to
  get past both.
- Assert the negative space as well as the positive: what must not happen, not just what should.
- Split compound assertions. `assert!(a); assert!(b);` says which one failed;
  `assert!(a && b)` doesn't.
- Check relationships between constants at compile time: `const _: () = assert!(A < B);`.
- **Control plane and data plane.** Use `assert!` on the control plane: configuration, logon,
  resends and store recovery, where one check costs nothing measurable. Use `debug_assert!` on the
  per-message data plane. It runs in every test and in the fuzz targets (cargo-fuzz enables debug
  assertions) and costs nothing in a release build. The benchmarks and the allocation budget
  (`tests/allocations.rs`) keep the data plane honest.

Assertions don't replace understanding the code. They check that the reader's model of it
matches what actually happens.

### Handle every error

Most catastrophic failures in distributed systems come from errors that were handled wrongly or
not at all ([Yuan et al., OSDI 2014](https://www.usenix.org/conference/osdi14/technical-sessions/presentation/yuan)).
In library code:

- Return errors, or handle them where they arise. Don't discard them.
- Use `unwrap` only in tests, benchmarks and examples; `clippy::unwrap_used` enforces this. Where
  an invariant rules out failure, use `expect` and name the invariant:
  `.expect("guarded by is_some")`.
- If a panic is the right response, as when a message passes 4 GiB, make it a deliberate panic
  with a message, not an overflow.

### Be explicit

- No `unsafe` code. The workspace denies it; an exception needs an `#[allow]` with a `reason`,
  and review.
- Convert between integer types with `From` and `TryFrom`, not `as`. Where truncation is the
  point, allow the lint on that statement with a reason. The cast lints are on.
- Build with no warnings. CI runs clippy with `-D warnings` on every feature combination.

### Keep functions short

A function should fit on a screen: about 70 lines, which clippy checks. Functions still over the
limit are marked `#[expect(clippy::too_many_lines)]` until they are split. Shape functions so that:

- the parent keeps the control flow (the `match` and the `if`s) and the state;
- helpers compute what the parent then acts on;
- leaf functions are pure where possible.

Declare variables in the smallest scope they need, and close to where they are used.

### Run at your own pace

Don't let outside events drive the work directly. The connection loop reads what has arrived,
processes it as a batch, and writes the replies in one go, with a bound on how much it does per
wake-up. The session itself is a sans-IO state machine with an injected clock, so it can be
tested, and fuzzed, without sockets or real time.

## Performance

- **Sketch first.** The largest gains come at design time, before there is anything to measure.
  A design note sketches the four resources (network, disk, memory and CPU), for both bandwidth
  and latency, and aims to be roughly right. Optimise the slowest resource first, weighted by how
  often it is used.
- **Separate the control plane from the data plane.** Logon, configuration and recovery can
  afford checks and allocation. The per-message path can't.
- **Batch.** Amortise system calls, disk writes and wake-ups over many messages: outbound
  messages go into one reused buffer and one write.
- **Allocate at setup, not per message.** This is the zero-allocation goal in the roadmap.
  `tests/allocations.rs` counts the allocations per order → ack exactly, and the build fails if
  the count changes.
- **Keep hot loops simple.** Put a hot loop in a function that takes plain values rather than
  `&self`, so both the compiler and the reader can see what it touches.
- **Measure what you change.** Benchmarks are criterion, compiled with fat LTO and one codegen
  unit so results don't depend on how code happens to be partitioned. Record before and after
  numbers in the commit message.

## Developer experience

### Names

- Say what a thing is or does, without abbreviations, in the terms FIX itself uses (`MsgSeqNum`,
  `ResendRequest`).
- Put qualifiers and units last, most significant first: `seq_num_next`, `timeout_logout`. Use
  `Duration` rather than a unit suffix where possible.
- An index, a count and a size are different things. Name them differently, and convert between
  them explicitly.
- Don't give one name two meanings in different contexts.

### Comments and commits

- Comments explain why. The code already says what it does.
- Write comments as sentences, with capitals and full stops.
- A commit message says why the change was made, and gives measurements when it's about
  performance. Pull request descriptions aren't in the history and `git blame` doesn't show them,
  so the reasoning belongs in the commits.
- A test says what it checks and how, in its name or a comment.

### Order

Put the important things first. A file is read top to bottom: the public type and its main entry
points come before the helpers.

## Where Turbojet differs from Tiger Style, and why

- **120 columns, not 100.** rustfmt enforces 120. Moving to 100 would reformat the whole
  repository, including the generated crates, and bury `git blame` for little gain. Prose and
  comments wrap at about 100.
- **`usize` for indexing.** Tiger Style avoids architecture-sized integers. Rust indexes with
  `usize`, so Turbojet uses it for indices and lengths, and fixed-size types for everything on the
  wire and in storage.
- **Few dependencies, not zero.** Turbojet uses tokio for I/O, chrono for time, rustls for TLS,
  and little else. A new dependency must justify itself in its pull request, and `cargo-deny`
  checks every dependency's licence and known advisories.
- **Heap allocation, for now.** Tiger Style allocates everything statically at startup. Turbojet
  works towards the same end (no allocation per message in steady state), but `Message` and the
  stores still allocate. The roadmap tracks the remaining allocations, and the allocation budget
  measures them.
- **Scripts in shell.** Tiger Style writes scripts in the project's own language. Turbojet's few
  scripts (`scripts/`) are short enough to stay in bash.

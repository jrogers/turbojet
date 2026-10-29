------------------------------ MODULE InFlight ------------------------------
(* The in-flight marker of a replicated session's incoming side. The counterparty sends        *)
(* messages 1..N. Before handing one over, the session commits the marker at it; once handled, *)
(* a commit moves the next incoming number past it and clears the marker. Opened again over a  *)
(* copy with a marker, the session moves the number back to it, so the counterparty resends   *)
(* from there, and the Window messages from it are flagged maybe_redelivered. Each commit is   *)
(* copied to the standby: synchronously (the hand-over waits for it) or asynchronously (queued *)
(* up to MaxQueued commits, as incoming-only commits add no outgoing numbers to the lag).      *)
EXTENDS Naturals, Sequences

CONSTANTS
    N,          \* messages the counterparty sends
    Window,     \* messages flagged from a marker (256 in the engine)
    Async,      \* asynchronous replication
    MaxQueued,  \* commits an asynchronous session may leave unacknowledged (MAX_QUEUED, 64)
    MaxFailovers,
    OnPromote,  \* "none"; "marker": promotion sets the marker at each session's next number;
                \* "sent": promotion stores BelowLogon, which the first logon after it turns
                \* into the counterparty's logon number: resends below it are flagged
    Resolve     \* "logon": BelowLogon resolves to the counterparty's logon number (the design);
                \* "next": to the new active's own next incoming number (a planted bug)

Nodes == {"a", "b"}
Other(n) == IF n = "a" THEN "b" ELSE "a"
Max(x, y) == IF x > y THEN x ELSE y
None == 0
BelowLogon == 1000

VARIABLES
    active,     \* the node whose session runs
    copy,       \* [Nodes -> [next, marker, flagTo]]: each node's stored incoming state (marker 0:
                \* none); flagTo: 0 none, BelowLogon (set at promotion), or the bound below which
                \* resends are flagged, until the next number passes it
    phase,      \* "closed" | "idle" | "marked" (marker committed, not handed) | "handed"
    queue,      \* commits sent to the standby, not yet applied
    flagUpTo,   \* messages below this are flagged maybe_redelivered (the window from a marker)
    handled,    \* what the application was handed, in order: [seq, flagged]
    failovers

vars == <<active, copy, phase, queue, flagUpTo, handled, failovers>>

Init ==
    /\ active = "a"
    /\ copy = [n \in Nodes |-> [next |-> 1, marker |-> None, flagTo |-> 0]]
    /\ phase = "idle"
    /\ queue = <<>>
    /\ flagUpTo = 1
    /\ handled = <<>>
    /\ failovers = 0

Next1 == copy[active].next
\* The highest message the counterparty has sent: at least every one handed.
Sent == IF handled = <<>> THEN 0 ELSE Max(Next1 - 1, LET hs == {handled[i].seq : i \in 1..Len(handled)} IN CHOOSE m \in hs : \A x \in hs : x <= m)
\* A commit: made in the active's copy, and sent to the standby.
Commit(state) ==
    /\ copy' = [copy EXCEPT ![active] = state]
    /\ queue' = Append(queue, state)

\* The marker at the next message, committed before it's handed over. Asynchronously, a commit
\* waits once MaxQueued are unacknowledged.
Mark ==
    /\ phase = "idle" /\ Next1 <= N
    /\ Async => Len(queue) < MaxQueued
    /\ Commit([next |-> Next1, marker |-> Next1, flagTo |-> copy[active].flagTo])
    /\ phase' = "marked"
    /\ UNCHANGED <<active, flagUpTo, handled, failovers>>

\* Handed over once the marker's commit is done: synchronously, once the standby has it too.
Hand ==
    /\ phase = "marked"
    /\ ~Async => queue = <<>>
    /\ handled' = Append(handled, [seq |-> Next1, flagged |-> Next1 < flagUpTo])
    /\ phase' = "handed"
    /\ UNCHANGED <<active, copy, queue, flagUpTo, failovers>>

\* Handled: the next number moves past it, the marker cleared.
Done ==
    /\ phase = "handed"
    /\ Async => Len(queue) < MaxQueued
    /\ Commit([next |-> Next1 + 1, marker |-> None,
               flagTo |-> IF copy[active].flagTo # BelowLogon /\ Next1 + 1 >= copy[active].flagTo
                          THEN 0 ELSE copy[active].flagTo])
    /\ phase' = "idle"
    /\ UNCHANGED <<active, flagUpTo, handled, failovers>>

\* The standby applies the oldest commit sent to it.
Apply ==
    /\ queue # <<>>
    /\ copy' = [copy EXCEPT ![Other(active)] = Head(queue)]
    /\ queue' = Tail(queue)
    /\ UNCHANGED <<active, phase, flagUpTo, handled, failovers>>

\* A session opened over `state`: a marker moves the next number back to it and flags the window.
\* The counterparty logs on at one past the last it sent, at least every message handed.
LogonNumber == Sent + 1
ResolvedBound(state) ==
    IF state.flagTo # BelowLogon THEN state.flagTo
    ELSE IF Resolve = "logon" THEN LogonNumber ELSE state.next
Opened(state) ==
    LET flagTo == ResolvedBound(state) IN
    IF state.marker # None
    THEN [next |-> state.marker, marker |-> state.marker, flagTo |-> flagTo,
          flag |-> Max(state.marker + Window, flagTo)]
    ELSE [next |-> state.next, marker |-> None, flagTo |-> flagTo, flag |-> flagTo]

\* The active crashes and restarts over its own copy, its session closed until the counterparty
\* logs on.
Restart ==
    /\ phase' = "closed" /\ queue' = <<>> /\ flagUpTo' = 0
    /\ UNCHANGED <<active, copy, handled, failovers>>

\* The counterparty logs on: the session opens over the active's copy, which is stored with what
\* opening changed (a marker's number, a resolved bound), and sent whole to the standby.
Logon ==
    LET o == Opened(copy[active]) IN
    /\ phase = "closed"
    /\ copy' = [n \in Nodes |-> [next |-> o.next, marker |-> o.marker, flagTo |-> o.flagTo]]
    /\ flagUpTo' = o.flag
    /\ phase' = "idle"
    /\ UNCHANGED <<active, queue, handled, failovers>>

\* The standby takes over with its copy; the old active becomes its standby, sent the copy whole.
\* MarkOnPromote: promotion sets the marker at the next number first.
Failover ==
    LET b == Other(active)
        marked == IF OnPromote = "marker" /\ copy[b].marker = None
                  THEN [copy[b] EXCEPT !.marker = copy[b].next]
                  ELSE copy[b]
        promoted == IF OnPromote = "sent" THEN [marked EXCEPT !.flagTo = BelowLogon] ELSE marked
    IN
    /\ failovers < MaxFailovers
    /\ active' = b
    /\ copy' = [copy EXCEPT ![b] = promoted]
    /\ flagUpTo' = 0
    /\ phase' = "closed" /\ queue' = <<>>
    /\ failovers' = failovers + 1
    /\ UNCHANGED handled

Next == Mark \/ Hand \/ Done \/ Apply \/ Restart \/ Logon \/ Failover

Spec == Init /\ [][Next]_vars

\* A message handed over again is flagged maybe_redelivered.
NoUnflaggedRepeat ==
    \A i, j \in 1..Len(handled) :
        (i < j /\ handled[i].seq = handled[j].seq) => handled[j].flagged

\* No copy counts a message received that the application wasn't handed (at least once).
NoLoss ==
    \A n \in Nodes : \A s \in 1..(copy[n].next - 1) : \E i \in 1..Len(handled) : handled[i].seq = s

\* Bound the run: each message handed at most three times.
Bound == Len(handled) <= 3 * N
=============================================================================

---------------------------- MODULE Replication ----------------------------
(* A model of turbojet-replica: one FIX session's outgoing sequence numbers, held by two     *)
(* nodes, an active copying commits to a standby (Replica), fenced by a lease and by epochs. *)
(* The counterparty is `wire`: the messages it has received, by sequence number. A second    *)
(* session, closed, stands for every session the active isn't using. Constants switch on the *)
(* design as built (Fencing, HoldWhole, Checks) or off, and plant bugs (CheckFirst,          *)
(* PlantUnchecked): see README.md for which reproduce each flaw the model found.             *)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
    Nodes,          \* two node names
    None,           \* no node
    Lag,            \* asynchronous bound (0: synchronous)
    Degrade,        \* ReplicatedStorage::degrading
    Fencing,        \* TRUE: SqlLease-like (expires, fenced in_sync); FALSE: ManualLease
    CheckFirst,     \* the failing-over instance reads in_sync before acquiring the lease
    Skew,           \* the lease's database may expire it before its holder's clock says so
    Chunked,        \* a whole session goes in two batches (over SNAPSHOT_BATCH_BYTES)
    HoldWhole,      \* the standby holds a session sent whole until its last part
    LosesStore,     \* the standby may restart with its store gone (memory, a replaced disk)
    StandbyCrashes, \* the standby process may crash and restart over its store
    Checks,         \* catch_up checks every session against the standby's incarnation, and
                    \* promotion needs the standby checked since it started
    PlantUnchecked, \* a failover that promotes without the check
    MaxSeq, MaxSeq2, MaxEpoch, MaxMsg, MaxBatches, MaxConn, MaxInc

VARIABLES
    role,       \* [Nodes -> {"active","standby","down"}]
    epochOf,    \* the epoch each active holds
    store,      \* [Nodes -> Seq(msg)]: durable copy, index = MsgSeqNum, 0 = no message stored
    wire,       \* what the counterparty has received, by MsgSeqNum
    fatal,      \* the counterparty got a MsgSeqNum lower than expected
    nextMsg,
    lholder, lepoch, lvalid, linsync,   \* the lease row
    belief,     \* [Nodes -> BOOLEAN]: the holder's own clock says the lease holds
    seen,       \* [Nodes -> BOOLEAN]: in_sync as read before acquiring (CheckFirst)
    rep,        \* the standby's Replica
    chan,       \* batches in flight to the standby, in order
    res,        \* result of each batch: "pending" | "ack" | "fail" | "stale"
    sess,       \* the active side's per-node session state
    peer,       \* the node the counterparty is logged on to: one at a time
    \* A second session, committed and replicated before the run, then closed: its next outgoing
    \* number - 1 on each node, what its counterparty has had, and whether it's behind.
    store2, wire2, behind2, fatal2,
    \* The incarnation each node's active last compared each session against (0: none), and
    \* the next incarnation a standby starting takes.
    chk1, chk2, nextInc

vars == <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync,
          belief, seen, rep, chan, res, sess, peer, store2, wire2, behind2, fatal2, chk1, chk2, nextInc>>
s2vars == <<store2, wire2, behind2, fatal2>>
chkvars == <<chk1, chk2, nextInc>>

Idle == [phase |-> "idle", ext |-> 0, wait |-> {}, peer |-> 0, inc |-> 0]
FreshSess == [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE,
              cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> Idle]
FreshRep(epoch, inc) == [epoch |-> epoch, conn |-> None, connId |-> 0, bound |-> FALSE, cno |-> 0,
                         promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, inc |-> inc, checked |-> FALSE]
\* A standby starting over its store: Replica::new, a new incarnation, having seen no epoch.
Restarted == [FreshRep(0, nextInc) EXCEPT !.connId = rep.connId]

Init ==
    \E a \in Nodes :
        /\ role = [n \in Nodes |-> IF n = a THEN "active" ELSE "standby"]
        /\ epochOf = [n \in Nodes |-> IF n = a THEN 1 ELSE 0]
        /\ store = [n \in Nodes |-> <<>>]
        /\ wire = <<>>
        /\ fatal = FALSE
        /\ nextMsg = 1
        /\ lholder = a /\ lepoch = 1 /\ lvalid = TRUE /\ linsync = TRUE
        /\ belief = [n \in Nodes |-> n = a]
        /\ seen = [n \in Nodes |-> FALSE]
        /\ rep = FreshRep(0, 1)
        /\ chan = <<>>
        /\ res = <<>>
        /\ sess = [n \in Nodes |-> FreshSess]
        /\ peer = [node |-> a, conn |-> 1]
        /\ store2 = [n \in Nodes |-> 1] /\ wire2 = 1
        /\ behind2 = [n \in Nodes |-> FALSE] /\ fatal2 = FALSE
        /\ chk1 = [n \in Nodes |-> 0] /\ chk2 = [n \in Nodes |-> 0] /\ nextInc = 2

Standby == {n \in Nodes : role[n] = "standby"}
SB == CHOOSE n \in Standby : TRUE
Last(s, k) == IF Len(s) <= k THEN s ELSE SubSeq(s, Len(s) - k + 1, Len(s))
\* What a bind compares: next outgoing, and asynchronously a digest of the last Lag slots.
SameSummary(a, b) == Len(a) = Len(b) /\ (Lag > 0 => Last(a, Lag) = Last(b, Lag))
FailPending == [i \in DOMAIN res |-> IF res[i] = "pending" THEN "fail" ELSE res[i]]
Connected(n) == Standby # {} /\ rep.conn = n /\ ~rep.promoted
HoldsLease(n) == lholder = n /\ lepoch = epochOf[n]

\* The batches of a session sent whole: the session as `snap`, with the bind folded in.
Whole(snap) ==
    IF Chunked /\ Len(snap) > 1
    THEN <<[kind |-> "w1", snap |-> SubSeq(snap, 1, Len(snap) \div 2)], [kind |-> "w2", snap |-> snap]>>
    ELSE <<[kind |-> "w", snap |-> snap]>>

(* Sends `bs` from n, numbered on from `cno`, returning nothing: updates chan and res, and   *)
(* leaves the new ids as Len(res)+1 .. Len(res)+Len(bs).                                     *)
Ids(bs) == {Len(res) + i : i \in 1..Len(bs)}
Send(n, bs, cno, conn) ==
    LET up == Connected(n)
        full == [i \in 1..Len(bs) |-> [bs[i] EXCEPT !.cno = cno + i]]
        tagged == [i \in 1..Len(bs) |-> [full[i] EXCEPT !.id = Len(res) + i]]
    IN /\ Len(res) + Len(bs) <= MaxBatches
       /\ chan' = IF up THEN chan \o [i \in 1..Len(bs) |->
                     tagged[i] @@ [ep |-> epochOf[n], conn |-> conn, from |-> n]]
                  ELSE chan
       /\ res' = res \o [i \in 1..Len(bs) |-> IF up THEN "pending" ELSE "fail"]

Rec(seq, m) == [kind |-> "rec", seq |-> seq, msg |-> m, cno |-> 0, id |-> 0, snap |-> <<>>]
W(b) == b @@ [cno |-> 0, id |-> 0, seq |-> 0, msg |-> 0]

\* The counterparty receives n's messages through `s`, gaps resent from n's store.
Externalize(n, s) ==
    IF s = 0 \/ peer.node # n \/ peer.conn # sess[n].job.peer THEN UNCHANGED <<wire, fatal, peer>> /\ UNCHANGED s2vars
    ELSE IF s <= Len(wire) THEN fatal' = TRUE /\ UNCHANGED wire
    ELSE wire' = wire \o SubSeq(store[n], Len(wire) + 1, s) /\ UNCHANGED fatal

---------------------------------------------------------------------------
(* The active's link *)

Connect(n) ==
    /\ role[n] = "active" /\ Standby # {} /\ ~rep.promoted /\ ~rep.applying
    /\ rep.conn # n /\ epochOf[n] >= rep.epoch /\ rep.connId < MaxConn
    /\ rep' = [rep EXCEPT !.epoch = epochOf[n], !.conn = n, !.connId = @ + 1, !.bound = FALSE, !.cno = 0]
    /\ chan' = <<>> /\ res' = FailPending
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

Disconnect ==
    /\ rep.conn # None /\ ~rep.applying
    /\ rep' = [rep EXCEPT !.conn = None, !.bound = FALSE]
    /\ chan' = <<>> /\ res' = FailPending
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

---------------------------------------------------------------------------
(* The active's session *)

\* Opening binds and compares summaries; a copy that differs is sent whole.
Open(n) ==
    LET s == sess[n] IN
    /\ role[n] = "active" /\ ~s.open /\ s.job.phase = "idle"
    /\ IF Connected(n) /\ epochOf[n] >= rep.epoch /\ SameSummary(store[SB], store[n]) /\ ~s.behind
       THEN /\ sess' = [sess EXCEPT ![n].open = TRUE, ![n].bound = TRUE, ![n].cno = 0,
                                    ![n].conn = rep.connId, ![n].queue = <<>>, ![n].drained = Len(store[n])]
            /\ rep' = [rep EXCEPT !.bound = TRUE, !.cno = 0]
            /\ chk1' = [chk1 EXCEPT ![n] = rep.inc]
            /\ UNCHANGED <<chan, res, peer>> /\ UNCHANGED s2vars
       ELSE LET bs == [i \in 1..Len(Whole(store[n])) |-> W(Whole(store[n])[i])] IN
            /\ Send(n, bs, 0, rep.connId)
            /\ chk1' = chk1
            /\ sess' = [sess EXCEPT ![n].job = [phase |-> "open", ext |-> 0, wait |-> Ids(bs), peer |-> peer.conn, inc |-> rep.inc],
                                    ![n].cno = Len(bs), ![n].conn = rep.connId,
                                    ![n].queue = <<>>, ![n].drained = Len(store[n])]
            /\ UNCHANGED rep
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, peer, chk2, nextInc>> /\ UNCHANGED s2vars

\* A commit: made locally at once, its batch sent (or the session whole, if behind).
Commit(n) ==
    LET s == sess[n] IN
    /\ role[n] = "active" /\ s.open /\ s.job.phase = "idle" /\ belief[n]
    /\ Len(store[n]) < MaxSeq
    /\ \E m \in {0, nextMsg} :
        /\ m <= MaxMsg
        /\ nextMsg' = IF m = 0 THEN nextMsg ELSE nextMsg + 1
        /\ store' = [store EXCEPT ![n] = Append(@, m)]
        /\ LET seq == Len(store[n]) + 1
               behind == ~s.bound \/ s.behind
           IN IF behind
              THEN LET bs == [i \in 1..Len(Whole(store'[n])) |-> W(Whole(store'[n])[i])] IN
                   /\ Send(n, bs, 0, rep.connId)
                   /\ sess' = [sess EXCEPT ![n].job = [phase |-> "whole", ext |-> seq, wait |-> Ids(bs), peer |-> peer.conn, inc |-> rep.inc],
                                           ![n].cno = Len(bs), ![n].conn = rep.connId,
                                           ![n].queue = <<>>, ![n].drained = seq]
              ELSE LET b == Len(res) + 1
                       q == Append(s.queue, [last |-> seq, id |-> b])
                       \* how many of the oldest queued batches this commit waits for
                       k == IF seq - s.drained <= Lag THEN 0
                            ELSE CHOOSE i \in 1..Len(q) : seq - q[i].last <= Lag
                                    /\ \A j \in 1..(i-1) : seq - q[j].last > Lag
                   IN
                   /\ Send(n, <<Rec(seq, m)>>, s.cno, s.conn)
                   /\ sess' = [sess EXCEPT ![n].job = [phase |-> "commit", ext |-> seq, peer |-> peer.conn, inc |-> 0,
                                                       wait |-> {q[i].id : i \in 1..k}],
                                           ![n].cno = @ + 1,
                                           ![n].queue = SubSeq(q, k + 1, Len(q)),
                                           ![n].drained = IF k = 0 THEN s.drained ELSE q[k].last]
    /\ UNCHANGED <<role, epochOf, wire, fatal, lholder, lepoch, lvalid, linsync, belief, seen, rep, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* The commit's (or opening's, or catch-up's) job ends once everything it waits for has answered.
Finish(n) ==
    LET s == sess[n]
        j == s.job
        ok == \A b \in j.wait : res[b] = "ack"
        stale == \E b \in j.wait : res[b] = "stale"
        \* fall_behind: degrading, the lease is told first (fenced), unless it was already
        degrades == Degrade /\ Fencing /\ ~stale /\ (s.oos \/ HoldsLease(n))
    IN
    /\ role[n] = "active" /\ j.phase # "idle"
    /\ \A b \in j.wait : res[b] # "pending"
    /\ IF ok
       THEN /\ chk1' = IF j.phase = "commit" THEN chk1 ELSE [chk1 EXCEPT ![n] = j.inc]
            /\ sess' = [sess EXCEPT ![n].job = Idle,
                          ![n].open = (j.phase # "catch"),
                          ![n].bound = IF j.phase = "commit" THEN @ ELSE TRUE,
                          ![n].behind = IF j.phase = "commit" THEN @ ELSE FALSE]
            /\ Externalize(n, j.ext)
            /\ UNCHANGED linsync
       ELSE IF degrades
       THEN /\ linsync' = FALSE /\ chk1' = chk1
            /\ sess' = [sess EXCEPT ![n].job = Idle, ![n].open = (j.phase # "catch"),
                                    ![n].bound = FALSE, ![n].behind = TRUE, ![n].oos = TRUE]
            /\ Externalize(n, j.ext)
       ELSE \* the commit fails: the session disconnects; its local commit stays
            /\ sess' = [sess EXCEPT ![n].job = Idle, ![n].open = FALSE, ![n].queue = <<>>,
                                    ![n].bound = IF stale THEN @ ELSE FALSE]
            /\ UNCHANGED <<wire, fatal, linsync, peer, chk1>> /\ UNCHANGED s2vars
    /\ UNCHANGED <<role, epochOf, store, nextMsg, lholder, lepoch, lvalid, belief, seen, rep, chan, res, peer, chk2, nextInc>> /\ UNCHANGED s2vars

Close(n) ==
    /\ role[n] = "active" /\ sess[n].open /\ sess[n].job.phase = "idle"
    /\ sess' = [sess EXCEPT ![n].open = FALSE, ![n].queue = <<>>]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, rep, chan, res, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* catch_up: a closed session behind is sent whole; none behind, the lease is told (fenced).
CatchUp(n) ==
    LET s == sess[n] IN
    /\ role[n] = "active" /\ ~s.open /\ s.job.phase = "idle"
    /\ (Degrade /\ s.behind) \/ (Checks /\ Connected(n) /\ chk1[n] # rep.inc)
    /\ LET bs == [i \in 1..Len(Whole(store[n])) |-> W(Whole(store[n])[i])] IN
       /\ Send(n, bs, 0, rep.connId)
       /\ sess' = [sess EXCEPT ![n].job = [phase |-> "catch", ext |-> 0, wait |-> Ids(bs), peer |-> peer.conn, inc |-> rep.inc],
                               ![n].cno = Len(bs), ![n].conn = rep.connId]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, rep, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

InSyncAgain(n) ==
    /\ role[n] = "active" /\ sess[n].oos /\ ~sess[n].behind /\ ~behind2[n] /\ HoldsLease(n)
    /\ linsync' = TRUE
    /\ sess' = [sess EXCEPT ![n].oos = FALSE]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, belief, seen, rep, chan, res, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

---------------------------------------------------------------------------
(* The standby's Replica *)

Apply ==
    /\ Standby # {} /\ chan # <<>> /\ ~rep.applying
    /\ LET b == Head(chan)
           sb == SB
           mine == store[sb]
           ack(r, st) == /\ res' = [res EXCEPT ![b.id] = "ack"] /\ chan' = Tail(chan)
                         /\ rep' = r /\ store' = st
       IN
       IF b.ep < rep.epoch
       THEN /\ res' = [res EXCEPT ![b.id] = "stale"] /\ chan' = Tail(chan)
            /\ UNCHANGED <<rep, store, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars
       ELSE IF b.kind = "w"
       THEN \* bind, then Op::Reset, written at once; the records wait for the commit
            /\ rep' = [rep EXCEPT !.epoch = b.ep, !.bound = TRUE, !.cno = 0, !.applying = TRUE, !.snap = b.snap]
            /\ store' = [store EXCEPT ![sb] = <<>>]
            /\ UNCHANGED <<chan, res, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars
       ELSE IF b.kind = "w1" /\ ~HoldWhole
       THEN /\ rep' = [rep EXCEPT !.epoch = b.ep, !.bound = TRUE, !.cno = 0, !.applying = TRUE, !.snap = b.snap]
            /\ store' = [store EXCEPT ![sb] = <<>>]
            /\ UNCHANGED <<chan, res>>
       ELSE IF b.kind = "w1"
       THEN \* held: nothing changes until the last part
            ack([rep EXCEPT !.epoch = b.ep, !.bound = TRUE, !.cno = b.cno], store)
       ELSE IF b.conn # rep.connId \/ ~rep.bound
       THEN \* a batch for a handle not bound: the connection ends
            /\ rep' = [rep EXCEPT !.conn = None, !.bound = FALSE]
            /\ chan' = <<>> /\ res' = FailPending
            /\ UNCHANGED store
       ELSE IF b.cno # rep.cno + 1
               \/ (b.kind = "rec" /\ b.seq # Len(mine) + 1)
               \/ (b.kind = "w2" /\ ~HoldWhole /\ Len(mine) > Len(b.snap))
       THEN \* diverged
            /\ res' = [res EXCEPT ![b.id] = "fail"] /\ chan' = Tail(chan)
            /\ rep' = [rep EXCEPT !.epoch = b.ep]
            /\ UNCHANGED store
       ELSE IF b.kind = "rec"
       THEN ack([rep EXCEPT !.epoch = b.ep, !.cno = b.cno], [store EXCEPT ![sb] = Append(mine, b.msg)])
       ELSE IF HoldWhole
       THEN \* w2, the last part: the reset written at once, the records at the commit
            /\ rep' = [rep EXCEPT !.epoch = b.ep, !.applying = TRUE, !.snap = b.snap]
            /\ store' = [store EXCEPT ![sb] = <<>>]
            /\ UNCHANGED <<chan, res>>
       ELSE \* w2: the rest of the session
            ack([rep EXCEPT !.epoch = b.ep, !.cno = b.cno], [store EXCEPT ![sb] = b.snap])
    /\ UNCHANGED <<role, epochOf, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* The batch's commit: the records stored after the reset.
ApplyCommit ==
    /\ Standby # {} /\ rep.applying
    /\ LET b == Head(chan) IN
       /\ store' = [store EXCEPT ![SB] = rep.snap]
       /\ rep' = [rep EXCEPT !.applying = FALSE, !.cno = b.cno]
       /\ res' = [res EXCEPT ![b.id] = "ack"]
       /\ chan' = Tail(chan)
    /\ UNCHANGED <<role, epochOf, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* The standby's process restarts over its store: Replica::new, having seen no epoch.
StandbyCrash ==
    /\ StandbyCrashes /\ Standby # {} /\ nextInc < MaxInc
    /\ rep' = Restarted /\ nextInc' = nextInc + 1
    /\ chan' = <<>> /\ res' = FailPending
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer, chk1, chk2>> /\ UNCHANGED s2vars

---------------------------------------------------------------------------
(* The lease and failover *)

Acquirable(b) == IF Fencing THEN lholder = None \/ ~lvalid \/ lholder = b ELSE TRUE

ReadInSync(b) ==
    /\ CheckFirst /\ role[b] = "standby"
    /\ seen' = [seen EXCEPT ![b] = linsync]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, rep, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* The standby's instance takes the lease, checks in_sync, and promotes its Replica.
Failover(b) ==
    /\ role[b] = "standby" /\ lepoch < MaxEpoch /\ Acquirable(b) /\ ~rep.applying
    /\ (Fencing => IF CheckFirst THEN seen[b] ELSE linsync)
    /\ (Checks /\ ~PlantUnchecked => rep.checked)
    /\ lholder' = b /\ lepoch' = lepoch + 1 /\ lvalid' = TRUE
    /\ belief' = [belief EXCEPT ![b] = TRUE]
    /\ rep' = [rep EXCEPT !.promoted = TRUE, !.conn = None]
    /\ chan' = <<>> /\ res' = FailPending
    \* promote: skip next outgoing ahead by the lag accepted
    /\ store' = [store EXCEPT ![b] = @ \o [i \in 1..Lag |-> 0]]
    /\ role' = [role EXCEPT ![b] = "active"]
    /\ epochOf' = [epochOf EXCEPT ![b] = lepoch + 1]
    /\ sess' = [sess EXCEPT ![b] = FreshSess]
    /\ store2' = [store2 EXCEPT ![b] = @ + Lag]
    /\ chk1' = [chk1 EXCEPT ![b] = 0] /\ chk2' = [chk2 EXCEPT ![b] = 0]
    /\ UNCHANGED <<wire, fatal, nextMsg, linsync, seen, peer, wire2, behind2, fatal2, nextInc>>

\* A failed acquisition under CheckFirst would leave the lease alone; one under acquire-first
\* that finds the standby out of sync holds it and doesn't promote.
AcquireOnly(b) ==
    /\ Fencing /\ role[b] = "standby" /\ lepoch < MaxEpoch /\ Acquirable(b) /\ ~linsync /\ ~CheckFirst
    /\ lholder' = b /\ lepoch' = lepoch + 1 /\ lvalid' = TRUE
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, linsync, belief, seen, rep, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

LocalExpire(n) ==
    /\ Fencing /\ belief[n]
    /\ belief' = [belief EXCEPT ![n] = FALSE]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, seen, rep, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* The database expires the lease: not before its holder's own clock does, unless skewed.
DBExpire ==
    /\ Fencing /\ lvalid
    /\ (Skew \/ lholder = None \/ ~belief[lholder])
    /\ lvalid' = FALSE
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, linsync, belief, seen, rep, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* SqlLease::renew checks holder and epoch, not expiry.
Renew(n) ==
    /\ Fencing /\ role[n] = "active" /\ HoldsLease(n)
    /\ lvalid' = TRUE /\ belief' = [belief EXCEPT ![n] = TRUE]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, linsync, seen, rep, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

---------------------------------------------------------------------------
(* Crashes *)

ActiveCrash(n) ==
    /\ role[n] = "active"
    /\ role' = [role EXCEPT ![n] = "down"]
    /\ sess' = [sess EXCEPT ![n] = FreshSess]
    /\ belief' = [belief EXCEPT ![n] = FALSE]
    /\ UNCHANGED <<epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, seen, rep, chan, res, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* Back as the standby of the other node, with a fresh Replica.
RestartStandby(n) ==
    /\ role[n] = "down" /\ Standby = {} /\ (\E o \in Nodes : role[o] = "active") /\ nextInc < MaxInc
    /\ role' = [role EXCEPT ![n] = "standby"]
    /\ rep' = Restarted /\ nextInc' = nextInc + 1
    /\ chan' = <<>> /\ res' = FailPending
    /\ UNCHANGED <<epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer, chk1, chk2>> /\ UNCHANGED s2vars

\* Back as the active, taking the lease again; the first catch-up reads in_sync.
RestartActive(n) ==
    /\ role[n] = "down" /\ lepoch < MaxEpoch /\ \A o \in Nodes : role[o] # "active"
    /\ (Fencing => lholder = n \/ lholder = None \/ ~lvalid)
    /\ role' = [role EXCEPT ![n] = "active"]
    /\ lholder' = n /\ lepoch' = lepoch + 1 /\ lvalid' = TRUE
    /\ epochOf' = [epochOf EXCEPT ![n] = lepoch + 1]
    /\ belief' = [belief EXCEPT ![n] = TRUE]
    /\ sess' = [sess EXCEPT ![n] = [FreshSess EXCEPT !.oos = ~linsync, !.behind = ~linsync]]
    /\ UNCHANGED <<store, wire, fatal, nextMsg, linsync, seen, rep, chan, res, peer>> /\ store2' = store2 /\ wire2' = wire2 /\ fatal2' = fatal2
    /\ behind2' = [behind2 EXCEPT ![n] = ~linsync]
    /\ chk1' = [chk1 EXCEPT ![n] = 0] /\ chk2' = [chk2 EXCEPT ![n] = 0] /\ nextInc' = nextInc

\* The counterparty logs on to n, its connection to the other node gone.
PeerSwitch(n) ==
    /\ role[n] = "active" /\ peer.node # n /\ peer.conn < MaxConn
    /\ peer' = [node |-> n, conn |-> peer.conn + 1]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, rep, chan, res, sess>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

(* The second session *)

\* The standby restarts with its store gone: a MemoryStorage, a replaced disk.
LoseStore ==
    /\ LosesStore /\ Standby # {} /\ ~rep.applying /\ nextInc < MaxInc
    /\ store' = [store EXCEPT ![SB] = <<>>]
    /\ store2' = [store2 EXCEPT ![SB] = 0]
    /\ rep' = Restarted /\ nextInc' = nextInc + 1
    /\ chan' = <<>> /\ res' = FailPending
    /\ UNCHANGED <<role, epochOf, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, sess, peer, wire2, behind2, fatal2, chk1, chk2>>

\* The holder opens the second session and sends a message. Opening compares the standby's copy
\* and sends it whole if it differs; degrading, a standby it can't reach puts the lease out of sync.
Use2(n) ==
    /\ role[n] = "active" /\ HoldsLease(n) /\ belief[n] /\ store2[n] < MaxSeq2
    /\ IF fatal2 = FALSE /\ store2[n] + 1 <= wire2 THEN fatal2' = TRUE ELSE fatal2' = fatal2
    /\ wire2' = IF store2[n] + 1 > wire2 THEN store2[n] + 1 ELSE wire2
    /\ IF Connected(n) /\ epochOf[n] >= rep.epoch /\ ~rep.applying
       THEN /\ store2' = [store2 EXCEPT ![n] = @ + 1, ![SB] = store2[n] + 1]
            /\ behind2' = [behind2 EXCEPT ![n] = FALSE]
            /\ chk2' = [chk2 EXCEPT ![n] = rep.inc]
            /\ UNCHANGED <<linsync, sess>>
       ELSE /\ Degrade /\ Fencing /\ (sess[n].oos \/ HoldsLease(n))
            /\ store2' = [store2 EXCEPT ![n] = @ + 1]
            /\ behind2' = [behind2 EXCEPT ![n] = TRUE]
            /\ linsync' = FALSE /\ sess' = [sess EXCEPT ![n].oos = TRUE]
            /\ chk2' = chk2
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, belief, seen, rep, chan, res, peer, chk1, nextInc>>

\* catch_up sends the closed second session whole when it's behind (degrading), or compares it
\* when it hasn't been against this incarnation (Checks), sending it whole if it differs.
CatchUp2(n) ==
    /\ role[n] = "active" /\ Connected(n) /\ epochOf[n] >= rep.epoch /\ ~rep.applying
    /\ (Degrade /\ behind2[n]) \/ (Checks /\ chk2[n] # rep.inc)
    /\ store2' = [store2 EXCEPT ![SB] = store2[n]]
    /\ behind2' = [behind2 EXCEPT ![n] = FALSE]
    /\ chk2' = [chk2 EXCEPT ![n] = rep.inc]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, rep, chan, res, sess, peer, wire2, fatal2, chk1, nextInc>>

\* catch_up marks an open session not checked against this incarnation behind, so its next
\* commit sends it whole.
MarkOpen(n) ==
    /\ Checks /\ role[n] = "active" /\ sess[n].open /\ ~sess[n].behind /\ Connected(n) /\ chk1[n] # rep.inc
    /\ sess' = [sess EXCEPT ![n].behind = TRUE]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, rep, chan, res, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

\* Every session checked against this incarnation and none behind: the Checked frame. The
\* standby takes it only if it names its own incarnation, which a guard on rep.inc models.
SendChecked(n) ==
    /\ Checks /\ role[n] = "active" /\ Connected(n) /\ epochOf[n] >= rep.epoch /\ ~rep.applying
    /\ chk1[n] = rep.inc /\ chk2[n] = rep.inc /\ ~sess[n].behind /\ ~behind2[n] /\ ~rep.checked
    /\ rep' = [rep EXCEPT !.checked = TRUE, !.epoch = epochOf[n]]
    /\ UNCHANGED <<role, epochOf, store, wire, fatal, nextMsg, lholder, lepoch, lvalid, linsync, belief, seen, chan, res, sess, peer>> /\ UNCHANGED s2vars /\ UNCHANGED chkvars

---------------------------------------------------------------------------

Next ==
    \/ LoseStore
    \/ Disconnect \/ Apply \/ ApplyCommit \/ StandbyCrash \/ DBExpire
    \/ \E n \in Nodes :
        \/ Connect(n) \/ Open(n) \/ Commit(n) \/ Finish(n) \/ Close(n)
        \/ CatchUp(n) \/ InSyncAgain(n)
        \/ ReadInSync(n) \/ Failover(n) \/ AcquireOnly(n)
        \/ LocalExpire(n) \/ Renew(n)
        \/ ActiveCrash(n) \/ RestartStandby(n) \/ RestartActive(n) \/ PeerSwitch(n) \/ Use2(n) \/ CatchUp2(n)
        \/ MarkOpen(n) \/ SendChecked(n)

Spec == Init /\ [][Next]_vars

\* The counterparty never gets a MsgSeqNum it has had already.
NoSeqReuse == ~fatal

\* The lease's current holder, asked to resend, sends what the counterparty had or a gap fill.
ResendsMatch ==
    \A n \in Nodes :
        (role[n] = "active" /\ HoldsLease(n)) =>
            \A i \in 1..Len(wire) : i <= Len(store[n]) => store[n][i] \in {0, wire[i]}

\* Each copy that may be promoted holds every number the counterparty has had, less the lag.
StandbyCovers ==
    (Standby # {} /\ ~rep.applying /\ linsync /\ \A n \in Nodes : sess[n].job.phase = "idle")
        => Len(store[SB]) + Lag >= Len(wire)
\* The second session never sends a number its counterparty has had.
NoSeqReuse2 == ~fatal2

\* While the lease says in sync, the standby holds the second session, less the lag.
StandbyCovers2 == (Standby # {} /\ linsync) => store2[SB] + Lag >= wire2
\* A standby that may be promoted (checked, and the lease in sync) holds what was sent of each
\* session, less the lag.
PromotableCovers ==
    (Standby # {} /\ rep.checked /\ linsync /\ ~rep.applying /\ \A n \in Nodes : sess[n].job.phase = "idle")
        => Len(store[SB]) + Lag >= Len(wire)
PromotableCovers2 == (Standby # {} /\ rep.checked /\ linsync) => store2[SB] + Lag >= wire2

\* Probes, each expected to be violated: the standby gets checked; a checked standby is promoted
\* after losing its store and being checked again; the promoted node sends on session 2.
ProbeChecked == ~rep.checked
ProbeRecheckedFailover == ~(nextInc > 2 /\ rep.promoted /\ \E n \in Nodes : role[n] = "active" /\ epochOf[n] = 2)
ProbeUse2AfterFailover == ~(lepoch = 2 /\ wire2 >= 2)
=============================================================================

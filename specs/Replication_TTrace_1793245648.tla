---- MODULE Replication_TTrace_1793245648 ----
EXTENDS Sequences, TLCExt, Replication_TEConstants, Toolbox, Replication, Naturals, TLC

_expression ==
    LET Replication_TEExpression == INSTANCE Replication_TEExpression
    IN Replication_TEExpression!expression
----

_trace ==
    LET Replication_TETrace == INSTANCE Replication_TETrace
    IN Replication_TETrace!trace
----

_inv ==
    ~(
        TLCGet("level") = Len(_TETrace)
        /\
        res = (<<>>)
        /\
        chk2 = ((a :> 3 @@ b :> 0))
        /\
        role = ((a :> "active" @@ b :> "standby"))
        /\
        chk1 = ((a :> 0 @@ b :> 0))
        /\
        store2 = ((a :> 1 @@ b :> 1))
        /\
        sess = ((a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]))
        /\
        lvalid = (TRUE)
        /\
        store = ((a :> <<>> @@ b :> <<>>))
        /\
        seen = ((a :> FALSE @@ b :> FALSE))
        /\
        fatal = (FALSE)
        /\
        wire = (<<>>)
        /\
        lepoch = (2)
        /\
        wire2 = (1)
        /\
        fatal2 = (TRUE)
        /\
        peer = ([conn |-> 1, node |-> b])
        /\
        nextMsg = (1)
        /\
        nextInc = (4)
        /\
        rep = ([inc |-> 3, bound |-> FALSE, cno |-> 0, conn |-> a, epoch |-> 2, connId |-> 1, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE])
        /\
        chan = (<<>>)
        /\
        belief = ((a :> TRUE @@ b :> FALSE))
        /\
        linsync = (TRUE)
        /\
        epochOf = ((a :> 2 @@ b :> 1))
        /\
        behind2 = ((a :> FALSE @@ b :> FALSE))
        /\
        lholder = (a)
    )
----

_init ==
    /\ sess = _TETrace[1].sess
    /\ wire2 = _TETrace[1].wire2
    /\ wire = _TETrace[1].wire
    /\ lvalid = _TETrace[1].lvalid
    /\ lepoch = _TETrace[1].lepoch
    /\ fatal2 = _TETrace[1].fatal2
    /\ linsync = _TETrace[1].linsync
    /\ chk1 = _TETrace[1].chk1
    /\ chk2 = _TETrace[1].chk2
    /\ store = _TETrace[1].store
    /\ store2 = _TETrace[1].store2
    /\ nextInc = _TETrace[1].nextInc
    /\ rep = _TETrace[1].rep
    /\ res = _TETrace[1].res
    /\ peer = _TETrace[1].peer
    /\ chan = _TETrace[1].chan
    /\ belief = _TETrace[1].belief
    /\ role = _TETrace[1].role
    /\ epochOf = _TETrace[1].epochOf
    /\ nextMsg = _TETrace[1].nextMsg
    /\ fatal = _TETrace[1].fatal
    /\ behind2 = _TETrace[1].behind2
    /\ lholder = _TETrace[1].lholder
    /\ seen = _TETrace[1].seen
----

_next ==
    /\ \E i,j \in DOMAIN _TETrace:
        /\ \/ /\ j = i + 1
              /\ i = TLCGet("level")
        /\ sess  = _TETrace[i].sess
        /\ sess' = _TETrace[j].sess
        /\ wire2  = _TETrace[i].wire2
        /\ wire2' = _TETrace[j].wire2
        /\ wire  = _TETrace[i].wire
        /\ wire' = _TETrace[j].wire
        /\ lvalid  = _TETrace[i].lvalid
        /\ lvalid' = _TETrace[j].lvalid
        /\ lepoch  = _TETrace[i].lepoch
        /\ lepoch' = _TETrace[j].lepoch
        /\ fatal2  = _TETrace[i].fatal2
        /\ fatal2' = _TETrace[j].fatal2
        /\ linsync  = _TETrace[i].linsync
        /\ linsync' = _TETrace[j].linsync
        /\ chk1  = _TETrace[i].chk1
        /\ chk1' = _TETrace[j].chk1
        /\ chk2  = _TETrace[i].chk2
        /\ chk2' = _TETrace[j].chk2
        /\ store  = _TETrace[i].store
        /\ store' = _TETrace[j].store
        /\ store2  = _TETrace[i].store2
        /\ store2' = _TETrace[j].store2
        /\ nextInc  = _TETrace[i].nextInc
        /\ nextInc' = _TETrace[j].nextInc
        /\ rep  = _TETrace[i].rep
        /\ rep' = _TETrace[j].rep
        /\ res  = _TETrace[i].res
        /\ res' = _TETrace[j].res
        /\ peer  = _TETrace[i].peer
        /\ peer' = _TETrace[j].peer
        /\ chan  = _TETrace[i].chan
        /\ chan' = _TETrace[j].chan
        /\ belief  = _TETrace[i].belief
        /\ belief' = _TETrace[j].belief
        /\ role  = _TETrace[i].role
        /\ role' = _TETrace[j].role
        /\ epochOf  = _TETrace[i].epochOf
        /\ epochOf' = _TETrace[j].epochOf
        /\ nextMsg  = _TETrace[i].nextMsg
        /\ nextMsg' = _TETrace[j].nextMsg
        /\ fatal  = _TETrace[i].fatal
        /\ fatal' = _TETrace[j].fatal
        /\ behind2  = _TETrace[i].behind2
        /\ behind2' = _TETrace[j].behind2
        /\ lholder  = _TETrace[i].lholder
        /\ lholder' = _TETrace[j].lholder
        /\ seen  = _TETrace[i].seen
        /\ seen' = _TETrace[j].seen

\* Uncomment the ASSUME below to write the states of the error trace
\* to the given file in Json format. Note that you can pass any tuple
\* to `JsonSerialize`. For example, a sub-sequence of _TETrace.
    \* ASSUME
    \*     LET J == INSTANCE Json
    \*         IN J!JsonSerialize("Replication_TTrace_1793245648.json", _TETrace)

=============================================================================

 Note that you can extract this module `Replication_TEExpression`
  to a dedicated file to reuse `expression` (the module in the 
  dedicated `Replication_TEExpression.tla` file takes precedence 
  over the module `Replication_TEExpression` below).

---- MODULE Replication_TEExpression ----
EXTENDS Sequences, TLCExt, Replication_TEConstants, Toolbox, Replication, Naturals, TLC

expression == 
    [
        \* To hide variables of the `Replication` spec from the error trace,
        \* remove the variables below.  The trace will be written in the order
        \* of the fields of this record.
        sess |-> sess
        ,wire2 |-> wire2
        ,wire |-> wire
        ,lvalid |-> lvalid
        ,lepoch |-> lepoch
        ,fatal2 |-> fatal2
        ,linsync |-> linsync
        ,chk1 |-> chk1
        ,chk2 |-> chk2
        ,store |-> store
        ,store2 |-> store2
        ,nextInc |-> nextInc
        ,rep |-> rep
        ,res |-> res
        ,peer |-> peer
        ,chan |-> chan
        ,belief |-> belief
        ,role |-> role
        ,epochOf |-> epochOf
        ,nextMsg |-> nextMsg
        ,fatal |-> fatal
        ,behind2 |-> behind2
        ,lholder |-> lholder
        ,seen |-> seen
        
        \* Put additional constant-, state-, and action-level expressions here:
        \* ,_stateNumber |-> _TEPosition
        \* ,_sessUnchanged |-> sess = sess'
        
        \* Format the `sess` variable as Json value.
        \* ,_sessJson |->
        \*     LET J == INSTANCE Json
        \*     IN J!ToJson(sess)
        
        \* Lastly, you may build expressions over arbitrary sets of states by
        \* leveraging the _TETrace operator.  For example, this is how to
        \* count the number of times a spec variable changed up to the current
        \* state in the trace.
        \* ,_sessModCount |->
        \*     LET F[s \in DOMAIN _TETrace] ==
        \*         IF s = 1 THEN 0
        \*         ELSE IF _TETrace[s].sess # _TETrace[s-1].sess
        \*             THEN 1 + F[s-1] ELSE F[s-1]
        \*     IN F[_TEPosition - 1]
    ]

=============================================================================



Parsing and semantic processing can take forever if the trace below is long.
 In this case, it is advised to uncomment the module below to deserialize the
 trace from a generated binary file.

\*
\*---- MODULE Replication_TETrace ----
\*EXTENDS IOUtils, Replication_TEConstants, Replication, TLC
\*
\*trace == IODeserialize("Replication_TTrace_1793245648.bin", TRUE)
\*
\*=============================================================================
\*

---- MODULE Replication_TETrace ----
EXTENDS Replication_TEConstants, Replication, TLC

trace == 
    <<
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "standby" @@ b :> "active"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 1 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 1,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 2,rep |-> [inc |-> 1, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> FALSE @@ b :> TRUE),linsync |-> TRUE,epochOf |-> (a :> 0 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> b]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "standby" @@ b :> "active"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 1,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 3,rep |-> [inc |-> 2, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> FALSE @@ b :> TRUE),linsync |-> TRUE,epochOf |-> (a :> 0 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> b]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "standby" @@ b :> "down"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 1,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 3,rep |-> [inc |-> 2, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> FALSE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 0 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> b]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "standby" @@ b :> "down"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> FALSE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 1,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 3,rep |-> [inc |-> 2, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> FALSE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 0 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> b]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "active" @@ b :> "down"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 2,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 3,rep |-> [inc |-> 2, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> TRUE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> TRUE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 2 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> a]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "active" @@ b :> "standby"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 2,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 4,rep |-> [inc |-> 3, bound |-> FALSE, cno |-> 0, conn |-> None, epoch |-> 0, connId |-> 0, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> TRUE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 2 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> a]),
    ([res |-> <<>>,chk2 |-> (a :> 0 @@ b :> 0),role |-> (a :> "active" @@ b :> "standby"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 0 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 2,wire2 |-> 1,fatal2 |-> FALSE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 4,rep |-> [inc |-> 3, bound |-> FALSE, cno |-> 0, conn |-> a, epoch |-> 2, connId |-> 1, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> TRUE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 2 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> a]),
    ([res |-> <<>>,chk2 |-> (a :> 3 @@ b :> 0),role |-> (a :> "active" @@ b :> "standby"),chk1 |-> (a :> 0 @@ b :> 0),store2 |-> (a :> 1 @@ b :> 1),sess |-> (a :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]] @@ b :> [open |-> FALSE, bound |-> FALSE, behind |-> FALSE, oos |-> FALSE, cno |-> 0, conn |-> 0, queue |-> <<>>, drained |-> 0, job |-> [peer |-> 0, phase |-> "idle", ext |-> 0, wait |-> {}, inc |-> 0]]),lvalid |-> TRUE,store |-> (a :> <<>> @@ b :> <<>>),seen |-> (a :> FALSE @@ b :> FALSE),fatal |-> FALSE,wire |-> <<>>,lepoch |-> 2,wire2 |-> 1,fatal2 |-> TRUE,peer |-> [conn |-> 1, node |-> b],nextMsg |-> 1,nextInc |-> 4,rep |-> [inc |-> 3, bound |-> FALSE, cno |-> 0, conn |-> a, epoch |-> 2, connId |-> 1, promoted |-> FALSE, applying |-> FALSE, snap |-> <<>>, checked |-> FALSE],chan |-> <<>>,belief |-> (a :> TRUE @@ b :> FALSE),linsync |-> TRUE,epochOf |-> (a :> 2 @@ b :> 1),behind2 |-> (a :> FALSE @@ b :> FALSE),lholder |-> a])
    >>
----


=============================================================================

---- MODULE Replication_TEConstants ----
EXTENDS Replication

CONSTANTS a, b

=============================================================================

---- CONFIG Replication_TTrace_1793245648 ----
CONSTANTS
    Nodes = { a , b }
    None = None
    Lag = 0
    Degrade = FALSE
    Fencing = TRUE
    CheckFirst = FALSE
    Skew = FALSE
    Chunked = FALSE
    HoldWhole = TRUE
    LosesStore = TRUE
    StandbyCrashes = FALSE
    Checks = FALSE
    PlantUnchecked = FALSE
    MaxSeq = 2
    MaxSeq2 = 2
    MaxEpoch = 2
    MaxMsg = 1
    MaxBatches = 5
    MaxConn = 2
    MaxInc = 4
    a = a
    b = b
    None = None

INVARIANT
    _inv

CHECK_DEADLOCK
    \* CHECK_DEADLOCK off because of PROPERTY or INVARIANT above.
    FALSE

INIT
    _init

NEXT
    _next

CONSTANT
    _TETrace <- _trace

ALIAS
    _expression
=============================================================================
\* Generated on Thu Oct 29 03:47:29 UTC 2026
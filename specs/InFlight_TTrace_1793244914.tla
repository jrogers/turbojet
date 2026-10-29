---- MODULE InFlight_TTrace_1793244914 ----
EXTENDS Sequences, TLCExt, Toolbox, Naturals, TLC, InFlight

_expression ==
    LET InFlight_TEExpression == INSTANCE InFlight_TEExpression
    IN InFlight_TEExpression!expression
----

_trace ==
    LET InFlight_TETrace == INSTANCE InFlight_TETrace
    IN InFlight_TETrace!trace
----

_inv ==
    ~(
        TLCGet("level") = Len(_TETrace)
        /\
        flagUpTo = (0)
        /\
        phase = ("handed")
        /\
        failovers = (1)
        /\
        handled = (<<[seq |-> 1, flagged |-> FALSE], [seq |-> 1, flagged |-> FALSE]>>)
        /\
        active = ("b")
        /\
        copy = ([b |-> [next |-> 1, marker |-> 1, flagTo |-> 0], a |-> [next |-> 1, marker |-> 0, flagTo |-> 0]])
        /\
        queue = (<<[next |-> 1, marker |-> 1, flagTo |-> 0]>>)
    )
----

_init ==
    /\ flagUpTo = _TETrace[1].flagUpTo
    /\ active = _TETrace[1].active
    /\ phase = _TETrace[1].phase
    /\ copy = _TETrace[1].copy
    /\ handled = _TETrace[1].handled
    /\ failovers = _TETrace[1].failovers
    /\ queue = _TETrace[1].queue
----

_next ==
    /\ \E i,j \in DOMAIN _TETrace:
        /\ \/ /\ j = i + 1
              /\ i = TLCGet("level")
        /\ flagUpTo  = _TETrace[i].flagUpTo
        /\ flagUpTo' = _TETrace[j].flagUpTo
        /\ active  = _TETrace[i].active
        /\ active' = _TETrace[j].active
        /\ phase  = _TETrace[i].phase
        /\ phase' = _TETrace[j].phase
        /\ copy  = _TETrace[i].copy
        /\ copy' = _TETrace[j].copy
        /\ handled  = _TETrace[i].handled
        /\ handled' = _TETrace[j].handled
        /\ failovers  = _TETrace[i].failovers
        /\ failovers' = _TETrace[j].failovers
        /\ queue  = _TETrace[i].queue
        /\ queue' = _TETrace[j].queue

\* Uncomment the ASSUME below to write the states of the error trace
\* to the given file in Json format. Note that you can pass any tuple
\* to `JsonSerialize`. For example, a sub-sequence of _TETrace.
    \* ASSUME
    \*     LET J == INSTANCE Json
    \*         IN J!JsonSerialize("InFlight_TTrace_1793244914.json", _TETrace)

=============================================================================

 Note that you can extract this module `InFlight_TEExpression`
  to a dedicated file to reuse `expression` (the module in the 
  dedicated `InFlight_TEExpression.tla` file takes precedence 
  over the module `InFlight_TEExpression` below).

---- MODULE InFlight_TEExpression ----
EXTENDS Sequences, TLCExt, Toolbox, Naturals, TLC, InFlight

expression == 
    [
        \* To hide variables of the `InFlight` spec from the error trace,
        \* remove the variables below.  The trace will be written in the order
        \* of the fields of this record.
        flagUpTo |-> flagUpTo
        ,active |-> active
        ,phase |-> phase
        ,copy |-> copy
        ,handled |-> handled
        ,failovers |-> failovers
        ,queue |-> queue
        
        \* Put additional constant-, state-, and action-level expressions here:
        \* ,_stateNumber |-> _TEPosition
        \* ,_flagUpToUnchanged |-> flagUpTo = flagUpTo'
        
        \* Format the `flagUpTo` variable as Json value.
        \* ,_flagUpToJson |->
        \*     LET J == INSTANCE Json
        \*     IN J!ToJson(flagUpTo)
        
        \* Lastly, you may build expressions over arbitrary sets of states by
        \* leveraging the _TETrace operator.  For example, this is how to
        \* count the number of times a spec variable changed up to the current
        \* state in the trace.
        \* ,_flagUpToModCount |->
        \*     LET F[s \in DOMAIN _TETrace] ==
        \*         IF s = 1 THEN 0
        \*         ELSE IF _TETrace[s].flagUpTo # _TETrace[s-1].flagUpTo
        \*             THEN 1 + F[s-1] ELSE F[s-1]
        \*     IN F[_TEPosition - 1]
    ]

=============================================================================



Parsing and semantic processing can take forever if the trace below is long.
 In this case, it is advised to uncomment the module below to deserialize the
 trace from a generated binary file.

\*
\*---- MODULE InFlight_TETrace ----
\*EXTENDS IOUtils, TLC, InFlight
\*
\*trace == IODeserialize("InFlight_TTrace_1793244914.bin", TRUE)
\*
\*=============================================================================
\*

---- MODULE InFlight_TETrace ----
EXTENDS TLC, InFlight

trace == 
    <<
    ([flagUpTo |-> 1,phase |-> "idle",failovers |-> 0,handled |-> <<>>,active |-> "a",copy |-> [b |-> [next |-> 1, marker |-> 0, flagTo |-> 0], a |-> [next |-> 1, marker |-> 0, flagTo |-> 0]],queue |-> <<>>]),
    ([flagUpTo |-> 1,phase |-> "marked",failovers |-> 0,handled |-> <<>>,active |-> "a",copy |-> [b |-> [next |-> 1, marker |-> 0, flagTo |-> 0], a |-> [next |-> 1, marker |-> 1, flagTo |-> 0]],queue |-> <<[next |-> 1, marker |-> 1, flagTo |-> 0]>>]),
    ([flagUpTo |-> 1,phase |-> "handed",failovers |-> 0,handled |-> <<[seq |-> 1, flagged |-> FALSE]>>,active |-> "a",copy |-> [b |-> [next |-> 1, marker |-> 0, flagTo |-> 0], a |-> [next |-> 1, marker |-> 1, flagTo |-> 0]],queue |-> <<[next |-> 1, marker |-> 1, flagTo |-> 0]>>]),
    ([flagUpTo |-> 0,phase |-> "closed",failovers |-> 1,handled |-> <<[seq |-> 1, flagged |-> FALSE]>>,active |-> "b",copy |-> [b |-> [next |-> 1, marker |-> 0, flagTo |-> 0], a |-> [next |-> 1, marker |-> 1, flagTo |-> 0]],queue |-> <<>>]),
    ([flagUpTo |-> 0,phase |-> "idle",failovers |-> 1,handled |-> <<[seq |-> 1, flagged |-> FALSE]>>,active |-> "b",copy |-> [b |-> [next |-> 1, marker |-> 0, flagTo |-> 0], a |-> [next |-> 1, marker |-> 0, flagTo |-> 0]],queue |-> <<>>]),
    ([flagUpTo |-> 0,phase |-> "marked",failovers |-> 1,handled |-> <<[seq |-> 1, flagged |-> FALSE]>>,active |-> "b",copy |-> [b |-> [next |-> 1, marker |-> 1, flagTo |-> 0], a |-> [next |-> 1, marker |-> 0, flagTo |-> 0]],queue |-> <<[next |-> 1, marker |-> 1, flagTo |-> 0]>>]),
    ([flagUpTo |-> 0,phase |-> "handed",failovers |-> 1,handled |-> <<[seq |-> 1, flagged |-> FALSE], [seq |-> 1, flagged |-> FALSE]>>,active |-> "b",copy |-> [b |-> [next |-> 1, marker |-> 1, flagTo |-> 0], a |-> [next |-> 1, marker |-> 0, flagTo |-> 0]],queue |-> <<[next |-> 1, marker |-> 1, flagTo |-> 0]>>])
    >>
----


=============================================================================

---- CONFIG InFlight_TTrace_1793244914 ----
CONSTANTS
    N = 3
    Window = 1
    Async = TRUE
    MaxQueued = 4
    MaxFailovers = 2
    OnPromote = "none"
    Resolve = "logon"

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
\* Generated on Thu Oct 29 03:35:14 UTC 2026
# turbojet-replica

Replicated session state for [Turbojet](https://github.com/jrogers/turbojet): an active instance
copies each FIX session's committed state to a hot standby, which can take the sessions over
with no sequence gap and no resend storm.

Work in progress: the wire format and the replicating store are being built.

Licensed under Apache-2.0.

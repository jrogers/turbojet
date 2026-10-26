# turbojet-replica

Replicated session state for [Turbojet](https://github.com/jrogers/turbojet): an active instance
copies each FIX session's committed state to a hot standby, which can take the sessions over
with no sequence gap and no resend storm.

`ReplicatedStorage` wraps the active's session store; each commit is made there and copied to the
standby, which applies it to a store of its own before acknowledging it, and nothing the commit
covers goes out until both have it. A lease, raised in epoch each time it changes hands, decides
which instance is active and fences off the other. See the crate documentation, and
[Storage and recovery](https://github.com/jrogers/turbojet/blob/main/docs/guide/storage.md#a-hot-standby)
in the guide.

Licensed under Apache-2.0.

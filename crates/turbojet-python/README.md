# turbojet-python

Python bindings for [Turbojet](https://github.com/jrogers/turbojet), to run FIX initiators from
Python's asyncio: the sessions a [`turbojet-config`](https://docs.rs/turbojet-config) file lists
under `[initiator.NAME]`, on the engine. It isn't on PyPI; build it into a virtualenv with
[maturin](https://www.maturin.rs) (Python 3.10 or later):

```sh
cd crates/turbojet-python
uv sync && uvx maturin develop --release
```

```python
import asyncio
import turbojet
from turbojet import AppMessage, Logon, Logout, Message, tags

async def main():
    async with turbojet.load("sessions.toml") as engine:
        async for event in engine.events():
            match event:
                case Logon(session):
                    order = Message("D", [(tags.CL_ORD_ID, "o1"), (tags.SYMBOL, "XYZ"), ...])
                    seq_num = await session.send(order)
                case AppMessage(session, msg):
                    if event.maybe_redelivered and already_handled(msg):
                        event.done()
                        continue
                    handle(msg)
                    event.done()
                case Logout(session, ended):
                    print(session.id, "ended:", ended)

asyncio.run(main())
```

Each application message arrives as an `AppMessage` and counts as received only once `done()` is
called: until then, a restart has the counterparty send it again, with `maybe_redelivered` set.
An event dropped without `done()`, by an exception say, stays unfinished, with a `RuntimeWarning`. A session reads nothing
more once it's 256 messages past one that isn't finished, so call `done()` for every message,
rejected ones included (`session.reject(msg, reason, text, ref_id)` sends a BusinessMessageReject).

Messages are untyped: fields by tag, values as text, `turbojet.tags` naming the tags, and
`msg.group(count_tag, member_tags)` reading a repeating group. Sessions also have
`send_nowait`, `logout`, `pause`/`resume` (awaited) and the sequence-number controls of
[Operating on sequence numbers](https://github.com/jrogers/turbojet#operating-on-sequence-numbers), and `engine.reload()` reloads the
file. Three callbacks must answer before the session goes on, so they're plain functions given to
`load`, run on the session's thread: keep them quick.

```python
turbojet.load(
    "sessions.toml",
    verify_logon=lambda session, logon, addr: None,   # or a reason to refuse it
    to_admin=add_credentials,                          # changes outbound session messages
    should_resend=lambda session, msg: True,           # False gap-fills it
)
turbojet.log_to_stderr("info")                         # the engine's logs
```

Acceptors and FIXP sessions aren't available from Python.

import asyncio
import gc

import pytest

import turbojet
from turbojet import AdminMessage, AppMessage, Logon, Logout, Message

from conftest import expect, sessions_file

DISK = """store = "disk"

[store.disk]
kind = "disk"
dir = "store"
"""


def order(cl_ord_id="order-1"):
    return Message(
        "D",
        [(11, cl_ord_id), (21, "1"), (55, "XYZ"), (54, "1"), (60, "20261017-12:00:00"), (38, "100"), (40, "1")],
    )



async def test_execution_report_arrives(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        logon = await expect(events, Logon)
        await logon.session.send(order())
        report = await expect(events, AppMessage)
        match report:
            case AppMessage(session, msg):
                assert msg.msg_type == "8"
                assert msg[11] == "order-1"
                assert session.id == logon.session.id
        assert not report.maybe_redelivered
        report.done()
        report.done()


async def test_unreleased_message_is_redelivered(gateway, tmp_path):
    path = sessions_file(tmp_path, gateway, DISK)
    async with turbojet.load(path) as engine:
        events = engine.events()
        logon = await expect(events, Logon)
        await logon.session.send(order())
        first = await expect(events, AppMessage)
        # Dropped unfinished, as by an exception: it stays held.
        cl_ord_id = first.msg[11]
        del first
        gc.collect()
    async with turbojet.load(path) as engine:
        again = await expect(engine.events(), AppMessage)
        assert again.msg[11] == cl_ord_id
        assert again.maybe_redelivered
        again.done()


async def test_reject(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        logon = await expect(events, Logon)
        await logon.session.send(order())
        report = await expect(events, AppMessage)
        assert await logon.session.reject(report.msg, 0, "not ours", ref_id=report.msg[11]) > 2
        report.done()


async def test_reject_needs_a_received_message(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        logon = await expect(engine.events(), Logon)
        with pytest.raises(ValueError):
            await logon.session.reject(order(), 0)


async def test_admin_message(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        logon = await expect(events, Logon)
        logon.session.logout()
        admin = await expect(events, AdminMessage)
        assert admin.msg.msg_type == "5"
        await expect(events, Logout)


async def test_verify_logon_refuses(gateway, tmp_path):
    seen = []

    def verify_logon(session, logon, addr):
        seen.append((session.id.target_comp_id, logon.msg_type, addr))
        return "not today"

    async with turbojet.load(sessions_file(tmp_path, gateway), verify_logon=verify_logon) as engine:
        with pytest.raises(asyncio.TimeoutError):
            await expect(engine.events(), Logon, timeout=1)
    assert seen[0] == ("GATEWAY", "A", f"127.0.0.1:{gateway}")


async def test_verify_logon_raising_refuses(gateway, tmp_path):
    def verify_logon(session, logon, addr):
        raise RuntimeError("boom")

    async with turbojet.load(sessions_file(tmp_path, gateway), verify_logon=verify_logon) as engine:
        with pytest.raises(asyncio.TimeoutError):
            await expect(engine.events(), Logon, timeout=1)


async def test_hooks_that_accept(gateway, tmp_path):
    sent = []

    def to_admin(session, msg):
        sent.append(msg.msg_type)
        if msg.msg_type == "A":
            msg[58] = "hello"

    async with turbojet.load(
        sessions_file(tmp_path, gateway),
        verify_logon=lambda session, logon, addr: None,
        to_admin=to_admin,
        should_resend=lambda session, msg: True,
    ) as engine:
        await expect(engine.events(), Logon)
    assert sent[0] == "A"


async def test_events_waiting_let_other_tasks_run(gateway, tmp_path):
    # Events already waiting are returned without a trip through the runtime, but a loop over
    # them still yields to the event loop now and then.
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        logon = await expect(events, Logon)
        for k in range(200):
            logon.session.send_nowait(order(f"order-{k}"))
        await asyncio.sleep(0.5)
        ticks = 0

        async def tick():
            nonlocal ticks
            while True:
                ticks += 1
                await asyncio.sleep(0)

        ticker = asyncio.create_task(tick())
        await asyncio.sleep(0)
        started = ticks

        async def receive(count):
            async for event in events:
                if isinstance(event, AppMessage):
                    event.done()
                    count -= 1
                    if count == 0:
                        return

        await asyncio.wait_for(receive(200), 10)
        ticker.cancel()
        assert ticks - started >= 5

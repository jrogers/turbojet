import asyncio

import pytest

import turbojet
from turbojet import Disconnect, Logon, Logout, Message

from conftest import free_port, sessions_file


def order():
    return Message(
        "D",
        [(11, "order-1"), (21, "1"), (55, "XYZ"), (54, "1"), (60, "20261017-12:00:00"), (38, "100"), (40, "1")],
    )


async def next_event(events):
    return await asyncio.wait_for(anext(events), 10)


async def test_logon_then_send(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        match await next_event(events):
            case Logon(session):
                assert session.id.target_comp_id == "GATEWAY"
                assert session.connected
            case other:
                pytest.fail(f"expected a Logon, got {other}")
        assert engine.sessions() == ["gateway"]
        assert await engine.session("gateway").send(order()) == 2


async def test_logout(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        logon = await next_event(events)
        logon.session.logout("done")
        match await next_event(events):
            case Logout(session, ended):
                assert session.id == logon.session.id
                assert ended == Disconnect.Logout
            case other:
                pytest.fail(f"expected a Logout, got {other}")


async def test_send_before_logon(tmp_path):
    async with turbojet.load(sessions_file(tmp_path, free_port())) as engine:
        with pytest.raises(turbojet.NotConnected):
            await engine.session("gateway").send(order())
        with pytest.raises(turbojet.NotConnected):
            engine.session("gateway").send_nowait(order())


async def test_events_end_on_exit(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        events = engine.events()
        await next_event(events)
    remaining = [event async for event in events]
    assert all(isinstance(event, (Logon, Logout)) for event in remaining)


def test_acceptor_refused(tmp_path):
    path = tmp_path / "sessions.toml"
    path.write_text('[acceptor]\nbegin_string = "FIX.4.2"\nsender_comp_id = "GATEWAY"\nlisten = "127.0.0.1:0"\n')
    with pytest.raises(turbojet.ConfigError, match="acceptor"):
        turbojet.load(path)


def test_bad_file(tmp_path):
    path = tmp_path / "sessions.toml"
    path.write_text("[initiator.x]\n")
    with pytest.raises(turbojet.ConfigError):
        turbojet.load(path)

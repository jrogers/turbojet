import turbojet
from turbojet import Logon, Logout, Message, SequenceNumbers

from conftest import expect, sessions_file


async def test_sequence_numbers(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        session = (await expect(engine.events(), Logon)).session
        numbers = await session.sequence_numbers()
        assert numbers == SequenceNumbers(next_incoming=2, next_outgoing=2)
        assert (await session.set_next_outgoing(10)).next_outgoing == 10
        assert await session.send(Message("D", [(11, "o1")])) == 10


async def test_pause_and_resume(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway, 'reconnect = { initial = "50ms", max = "100ms" }')) as engine:
        events = engine.events()
        session = (await expect(events, Logon)).session
        await session.pause("maintenance")
        assert session.paused
        await expect(events, Logout)
        await session.resume()
        assert not session.paused
        await expect(events, Logon)


async def test_reload(gateway, tmp_path):
    async with turbojet.load(sessions_file(tmp_path, gateway)) as engine:
        await expect(engine.events(), Logon)
        engine.reload()


def test_log_to_stderr():
    turbojet.log_to_stderr("warn")

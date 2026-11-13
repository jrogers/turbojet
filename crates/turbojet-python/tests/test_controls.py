import subprocess
import sys
from pathlib import Path

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


def test_log_to_python(gateway, tmp_path):
    # Its own process: logging is set once per process, and test_log_to_stderr sets it here.
    script = f"""
import asyncio, logging, turbojet
from conftest import expect

records = []
handler = logging.Handler()
handler.emit = records.append
logging.getLogger("turbojet").addHandler(handler)
logging.getLogger("turbojet").setLevel(logging.INFO)
turbojet.log_to_python()

async def main():
    async with turbojet.load({str(sessions_file(tmp_path, gateway))!r}) as engine:
        await expect(engine.events(), turbojet.Logon)

asyncio.run(main())
assert any(r.name == "turbojet.session" and r.getMessage().startswith("logged on") for r in records), records
assert not any(r.levelno < logging.INFO for r in records)
try:
    turbojet.log_to_stderr("info")
    raise AssertionError("log_to_stderr after log_to_python")
except turbojet.Error:
    pass
"""
    subprocess.run([sys.executable, "-c", script], cwd=Path(__file__).parent, check=True)

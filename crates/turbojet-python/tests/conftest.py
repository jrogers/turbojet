"""A counterparty for the tests: the engine's `gateway` example, a FIX 4.2 acceptor."""

import socket
import subprocess
import time
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[3]


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


@pytest.fixture(scope="session")
def gateway_binary():
    subprocess.run(["cargo", "build", "-q", "--example", "gateway", "--all-features"], cwd=ROOT, check=True)
    return ROOT / "target" / "debug" / "examples" / "gateway"


@pytest.fixture
def gateway(gateway_binary):
    """The gateway's port, once it's listening; it accepts CLIENT1."""
    port = free_port()
    proc = subprocess.Popen(
        [gateway_binary, "--listen", f"127.0.0.1:{port}", "--comp-id", "GATEWAY", "--allow", "CLIENT1"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    deadline = time.monotonic() + 10
    while True:
        try:
            socket.create_connection(("127.0.0.1", port), timeout=0.1).close()
            break
        except OSError:
            if time.monotonic() > deadline or proc.poll() is not None:
                proc.kill()
                raise RuntimeError("the gateway didn't start")
            time.sleep(0.05)
    yield port
    proc.kill()
    proc.wait()


def sessions_file(tmp_path, port, extra=""):
    """A sessions file with one initiator, `gateway`, logging on to `port` as CLIENT1."""
    path = tmp_path / "sessions.toml"
    path.write_text(
        f"""
[initiator.gateway]
begin_string = "FIX.4.2"
sender_comp_id = "CLIENT1"
target_comp_id = "GATEWAY"
connect = ["127.0.0.1:{port}"]
{extra}
"""
    )
    return path

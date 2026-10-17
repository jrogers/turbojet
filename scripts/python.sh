#!/usr/bin/env bash
# Checks the Python bindings (crates/turbojet-python), which are outside the workspace: formatting,
# clippy, then a build with maturin into the crate's virtualenv and its pytest suite, which runs
# against the engine's gateway example. Needs uv. Arguments go to pytest:
#   scripts/python.sh               # everything
#   scripts/python.sh -k logout     # tests matching "logout"
set -euo pipefail
cd "$(dirname "$0")/../crates/turbojet-python"

cargo fmt --check
uv sync --locked --no-install-project --quiet
# PyO3's build finds Python on PATH: the virtualenv's, through uv run.
uv run --no-sync cargo clippy --all-targets --locked --quiet -- -D warnings
uvx maturin develop --locked --quiet
uv run --no-sync pytest -q "$@"

#!/usr/bin/env bash
set -euo pipefail

export RUST_BACKTRACE=1

# Run wasm tests in headless Firefox via wasm-pack
wasm-pack test --headless --firefox

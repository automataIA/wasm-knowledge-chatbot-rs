#!/usr/bin/env bash
set -euo pipefail

export WASM_BINDGEN_TEST_BROWSER="firefox"
export WASM_BINDGEN_TEST_TIMEOUT="120"
export RUST_BACKTRACE=1

cargo test --target wasm32-unknown-unknown -- --headless

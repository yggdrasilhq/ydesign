#!/usr/bin/env bash
set -euo pipefail

# ygg-ci runs this from a non-login shell: put the rust toolchain on PATH.
export PATH="$PATH:$HOME/.cargo/bin"
cd "$(dirname "$0")/../specimens"
cargo test --locked --lib
dx build --web --release

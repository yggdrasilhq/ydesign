#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../specimens"
cargo test --locked --lib
dx build --web --release

#!/usr/bin/env sh
# Everything CI checks, stopping at the first failure. Run before committing.
set -e
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test

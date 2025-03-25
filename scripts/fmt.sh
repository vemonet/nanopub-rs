#!/usr/bin/env bash
set -e

cargo fmt
cargo clippy --all --all-targets --all-features

# uvx pre-commit run --all --all-files
# cargo deny check

#!/usr/bin/env bash

set -e

cd lib

cargo fmt -- --check

# cargo clippy --all --all-targets --all-features

# cargo test --verbose --all --all-features
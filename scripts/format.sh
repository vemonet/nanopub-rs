#!/usr/bin/env bash

set -e

cd lib
cargo fmt

# cargo clippy --all --all-targets --all-features
# cargo test --verbose --all --all-features


cd ../python
cargo fmt


cd ../js
cargo fmt

#!/usr/bin/env bash

set -e

source scripts/docs-build.sh

echo "📖 Rust doc at http://0.0.0.0:3000/doc/nanopub"

python -m http.server 3000 --directory ./target/doc

# cargo doc --no-deps --open
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc

# python3 -m webbrowser ./target/doc/

#!/usr/bin/env bash

set -e

rm -rf target/doc

mdbook build

cargo doc --workspace --no-deps --exclude nanopub-js --exclude nanopub-py --target-dir target/doc

echo "📖 Docs generated in the target/doc folder"
echo "MdBook at http://0.0.0.0:3000"
echo "Rust doc at http://0.0.0.0:3000/doc/nanopub"

python -m http.server 3000 --directory ./target/doc

# mdbook serve --open

# cargo doc --no-deps --open
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc

# python3 -m webbrowser ./target/doc/

#!/usr/bin/env bash

set -e

rm -rf target/doc

mdbook build --open

cargo doc --open --workspace --no-deps --exclude try-nanopub-rs --exclude nanopub-js --exclude nanopub_rs --target-dir target/doc

echo "📖 Docs generated in the target/doc folder"


# mdbook serve --open

# cargo doc --no-deps --open
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps
# # rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc

# python3 -m webbrowser ./docs/target/doc/nanopub_rs/index.html

# file:///home/vemonet/develop/perso/nanopub-rs/lib/docs/target/doc/nanopub_rs/index.html

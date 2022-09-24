#!/usr/bin/env bash

set -e

rm -rf target/doc

mdbook build

cargo doc --workspace --no-deps --exclude try-nanopub-rs --exclude nanopub-js --exclude nanopub_py --target-dir target/doc

echo "📖 Docs generated in the target/doc folder"

# rustdoc --extend-css custom.css src/lib.rs
# rustdoc --theme awesome.css src/lib.rs
# https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/static/css/themes/ayu.css
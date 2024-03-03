#!/usr/bin/env bash
set -e

rm -rf target/doc

mdbook build

# NOTE: we can also add the docstrings to the generated docs under /doc/nanopub, but it is already available at https://docs.rs/nanopub
# cargo doc --workspace --no-deps --exclude nanopub-cli --exclude nanopub-js --exclude nanopub-sign --target-dir target/doc


echo "Docs generated in the target/doc folder"

# rustdoc --extend-css custom.css src/lib.rs
# rustdoc --theme awesome.css src/lib.rs
# https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/static/css/themes/ayu.css

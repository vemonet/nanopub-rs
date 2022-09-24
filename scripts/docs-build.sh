#!/usr/bin/env bash

set -e

mdbook build
# mdbook build docs --dest-dir target

# cd lib
# rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps

# rustdoc --extend-css custom.css src/lib.rs
# rustdoc --theme awesome.css src/lib.rs
# https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/static/css/themes/ayu.css
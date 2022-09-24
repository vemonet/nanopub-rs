#!/usr/bin/env bash

set -e

cd lib

rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps

# python3 -m webbrowser ./docs/target/doc/index.html


mdbook serve docs --dest-dir target/mdbook --open

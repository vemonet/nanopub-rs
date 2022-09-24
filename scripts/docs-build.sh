#!/usr/bin/env bash

set -e

cd lib

mdbook build
# mdbook build docs --dest-dir target

# rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps
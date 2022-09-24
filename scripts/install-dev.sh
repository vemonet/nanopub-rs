#!/usr/bin/env bash

set -e

pip install maturin

cargo install mdbook

rustup component add rustfmt


echo "🦪 Make sure you have perl installed on your machine (required for openssl)"
# cf. https://github.com/openssl/openssl/issues/13761
# dnf install perl

cargo install wasm-pack
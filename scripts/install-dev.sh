#!/usr/bin/env bash

set -e

# Script to install dependencies for development and enable pre-commit hooks
# Make sure you have enabled a venv before running this script

pip install maturin pre-commit

cargo install mdbook

rustup component add rustfmt


echo "🦪 Make sure you have perl installed on your machine (required for openssl)"
# cf. https://github.com/openssl/openssl/issues/13761
# dnf install perl

cargo install wasm-pack

pre-commit install

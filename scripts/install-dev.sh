#!/usr/bin/env bash

set -e

# Script to install dependencies for development and enable pre-commit hooks
# Make sure you have enabled a venv before running this script

python3 -m venv .venv
source .venv/bin/activate

pip install maturin pre-commit

rustup update
rustup component add rustfmt clippy

cargo install wasm-pack mdbook

pre-commit install

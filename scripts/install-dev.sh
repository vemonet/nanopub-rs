#!/usr/bin/env bash

set -e

# Script to install dependencies for development and enable pre-commit hooks
# Make sure you have enabled a venv before running this script

pip install maturin pre-commit

cargo install mdbook

rustup update
rustup component add rustfmt clippy

cargo install wasm-pack

pre-commit install

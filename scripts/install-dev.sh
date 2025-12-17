#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

rustup update
cargo install wasm-pack cargo-release cargo-tarpaulin cargo-deny cargo-make git-cliff

uvx pre-commit install

# if [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
#     echo "Installing Linux specific dependency"
#     uv run pip install maturin[patchelf]
# fi

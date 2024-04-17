#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

python3 -m venv .venv
source .venv/bin/activate

pip install --upgrade pip
pip install -r python/requirements.txt
pip install -r lib/docs/requirements.txt

if [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "Installing Linux specific dependency"
    pip install maturin[patchelf]
fi

rustup update
cargo install wasm-pack cargo-tarpaulin cargo-deny cargo-make git-cliff

pre-commit install

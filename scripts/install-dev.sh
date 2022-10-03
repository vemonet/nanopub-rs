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

echo "🔮 You might need to install dependency for OpenSSL:"
echo "Ubuntu: sudo apt install -y pkg-config libssl-dev"
echo "Fedora: sudo dnf install -y pkg-config openssl-devel"
# echo ""
# echo "You might also need to set the OPENSSL_DIR environment variable:"
# echo "export OPENSSL_DIR=/opt/conda/ssl"

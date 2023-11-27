#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

rustup update

cargo install mdbook mdbook-admonish mdbook-pagetoc

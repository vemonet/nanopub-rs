#!/usr/bin/env bash
set -e

uvx pre-commit run --all --all-files
cargo deny check

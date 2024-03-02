#!/usr/bin/env bash
set -e

cargo test

./scripts/test-js.sh
./scripts/test-python.sh

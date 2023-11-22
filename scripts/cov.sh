#!/usr/bin/env bash
set -e

cargo tarpaulin -p nanopub --out html --timeout 120 \
    --exclude-files lib/src/error.rs

python -m http.server 3000 --directory .

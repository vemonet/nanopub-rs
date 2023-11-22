#!/usr/bin/env bash
set -e

source .venv/bin/activate
cd python

maturin develop

python try.py

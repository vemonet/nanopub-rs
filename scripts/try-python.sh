#!/usr/bin/env bash

set -e

echo "📦️ Make sure to have enabled your virtual environment"

cd python

maturin develop

python try.py
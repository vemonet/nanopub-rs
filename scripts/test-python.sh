#!/usr/bin/env bash
set -e

echo "Activating virtual environment"
source .venv/bin/activate

cd python

python -m pip install pytest maturin

maturin develop

python -m pytest -s

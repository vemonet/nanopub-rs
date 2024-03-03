#!/usr/bin/env bash
set -e

if [ ! -d ".venv" ]; then
    echo ".venv virtual environment does not exist. Creating it"
    python -m venv .venv
fi

echo "Activating virtual environment"
source .venv/bin/activate

cd python

python -m pip install pytest maturin

maturin develop

python -m pytest -s

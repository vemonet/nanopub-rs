#!/usr/bin/env bash
set -e

if [ -d ".venv" ]; then
    echo "Activating virtual environment"
    source .venv/bin/activate
else
    echo ".venv directory does not exist running without virtual environment"
fi

cd python

python -m pip install pytest maturin

maturin develop

python -m pytest -s

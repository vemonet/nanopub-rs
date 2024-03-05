#!/usr/bin/env bash
set -e

# Check for --no-build flag
SKIP_BUILD=false
for arg in "$@"; do
    if [[ $arg == "--no-build" ]]; then
        SKIP_BUILD=true
        break
    fi
done

if [ ! -d ".venv" ]; then
    echo ".venv virtual environment does not exist. Creating it"
    python -m venv .venv
fi

echo "Activating virtual environment"
source .venv/bin/activate

cd python

python -m pip install pytest maturin

if [ "$SKIP_BUILD" = false ]; then
    maturin develop
fi

python -m pytest -s

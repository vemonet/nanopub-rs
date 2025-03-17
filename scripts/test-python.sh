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

cd python

if [ "$SKIP_BUILD" = false ]; then
    uv run maturin develop --uv
    # uv run maturin develop
fi

uv run pytest -s

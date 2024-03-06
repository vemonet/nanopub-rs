#!/usr/bin/env bash
set -e

# source scripts/docs-build.sh

# echo "🦀 Rust doc at http://0.0.0.0:3000/doc/nanopub"
# echo "📖 MdBook at http://0.0.0.0:3000"

# python -m http.server 3000 --directory ./target/doc

# python3 -m webbrowser ./target/doc/


# With mkdocs:

# pip install -r lib/docs/requirements.txt

if [ ! -d ".venv" ]; then
    echo ".venv virtual environment does not exist. Creating it"
    python -m venv .venv
fi

echo "Activating virtual environment"
source .venv/bin/activate

pip install -q -r lib/docs/requirements.txt

mkdocs serve -a localhost:8001 -f lib/docs/mkdocs.yml

#!/usr/bin/env bash
set -e

# Start mkdocs in development

if [ ! -d ".venv" ]; then
    echo ".venv virtual environment does not exist. Creating it"
    python -m venv .venv
fi

echo "Activating virtual environment"
source .venv/bin/activate

pip install -q -r lib/docs/requirements.txt

mkdocs serve -a localhost:8001 -f lib/docs/mkdocs.yml

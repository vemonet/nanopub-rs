#!/usr/bin/env bash
set -e

# Start mkdocs in development

# uv run mkdocs serve -a localhost:8001 -f lib/docs/mkdocs.yml

cd lib/docs
uv run mkdocs serve -a localhost:8001

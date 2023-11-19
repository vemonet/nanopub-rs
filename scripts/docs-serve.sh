#!/usr/bin/env bash
set -e

source scripts/docs-build.sh

echo "🦀 Rust doc at http://0.0.0.0:3000/doc/nanopub"
echo "📖 MdBook at http://0.0.0.0:3000"

python -m http.server 3000 --directory ./target/doc

# python3 -m webbrowser ./target/doc/

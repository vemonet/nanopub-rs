#!/usr/bin/env bash
set -e

# ./scripts/test-js.sh

cd js

# npm install
# npm run test


# wasm-pack build --target web
# cargo build --target=wasm32-unknown-unknown

# python3 -m webbrowser http://0.0.0.0:8000

python3 -m http.server
# Or npm run start

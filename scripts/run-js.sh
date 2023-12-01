#!/usr/bin/env bash
set -e

cd js

npm run build

# wasm-pack build --target web
# cargo build --target=wasm32-unknown-unknown

# python3 -m webbrowser http://0.0.0.0:8000

python3 -m http.server
# Or npm run start

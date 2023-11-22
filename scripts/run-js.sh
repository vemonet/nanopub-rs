#!/usr/bin/env bash
set -e

cd js

# Or: npm run build
# npm run start

# --target x86_64-unknown-linux-gnu
wasm-pack build --target web

# cargo build --target=wasm32-unknown-unknown

python3 -m webbrowser http://0.0.0.0:8000

python3 -m http.server

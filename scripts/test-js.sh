#!/usr/bin/env bash
set -e

cd js

# Check for --no-build flag
SKIP_BUILD=false
for arg in "$@"; do
    if [[ $arg == "--no-build" ]]; then
        SKIP_BUILD=true
        break
    fi
done

npm install

if [ "$SKIP_BUILD" = false ]; then
    npm run test
else
    npm run jest
fi

# wasm-pack build --target web
# cargo build --target=wasm32-unknown-unknown

# python3 -m webbrowser http://0.0.0.0:8000

# python3 -m http.server
# Or npm run start

#!/usr/bin/env bash

set -e

cd js

wasm-pack build --target web


python3 -m webbrowser http://0.0.0.0:8000

python3 -m http.server


#!/usr/bin/env bash

set -e
bold=$(tput bold)
normal=$(tput sgr0)

echo "        ${bold}🦀 Trying the Rust lib 🦀${normal}"
./scripts/run-lib.sh

echo ""
echo "        ${bold}🐍 Trying the Python bindings 🐍${normal}"
./scripts/run-python.sh

echo ""
echo "        ${bold}☕️ Trying the JavaScript bindings ☕️${normal}"
./scripts/run-js.sh

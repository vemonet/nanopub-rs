#!/usr/bin/env bash

set -e

bold=$(tput bold)
normal=$(tput sgr0)

# Format all if no arg passed
PROCESS=( "lib" "python" "js" )

if [ ! -z "$1" ]
then
  PROCESS=( $1)
fi

for folder in ${PROCESS[@]}; do
    cd $folder
    echo ""
    if [ $folder == "lib" ] ;then
        echo "        ${bold}🦀 Testing the Rust lib 🦀${normal}"
    elif [ $folder == "python" ]; then
        echo "        ${bold}🐍 Testing the Python bindings 🐍${normal}"
    elif [ $folder == "js" ]; then
        echo "        ${bold}☕️ Testing the JavaScript bindings ☕️${normal}"
    fi
    echo ""

    cargo fmt -- --check
    cargo clippy --all --all-targets --all-features
    # cargo build
    cargo test --verbose --all --all-features
    cd ..
done

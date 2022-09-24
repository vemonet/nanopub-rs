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
    cargo fmt
    cd ..

    if [ $folder == "lib" ] ;then
        echo "🦀 Formatted the Rust lib"
    elif [ $folder == "python" ]; then
        echo "🐍 Formatted the Python bindings"
    elif [ $folder == "js" ]; then
        echo "☕️ Formatted the JavaScript bindings"
    fi
done

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
    if [ $folder == "lib" ] ;then
        echo "${bold}🦀 Formatting the Rust lib 🦀${normal}"
    elif [ $folder == "python" ]; then
        echo "${bold}🐍 Formatting the Python bindings 🐍${normal}"
    elif [ $folder == "js" ]; then
        echo "${bold}☕️ Formatting the JavaScript bindings ☕️${normal}"
    fi
    cd $folder
    cargo fmt
    cd ..
done

#!/usr/bin/env bash
set -e

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

new_version=$1
files=(
    "lib/Cargo.toml"
    "cli/Cargo.toml"
    "python/Cargo.toml"
    "js/Cargo.toml"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "$file"
        sed -i "s/nanopub = { version = \"[0-9]*\.[0-9]*\.[0-9]*\"/nanopub = { version = \"$new_version\"/" "$file"
        echo "🏷️  Updated version in $file"
    else
        echo "⚠️ File not found: $file"
    fi
done

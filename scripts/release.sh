#!/usr/bin/env bash
set -e

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi
new_version=$1

echo ""
echo "  🏔️ Update version in Cargo.toml"
echo ""
files=(
    "lib/Cargo.toml"
    "cli/Cargo.toml"
    "python/Cargo.toml"
    "js/Cargo.toml"
)
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        sed -i "s/nanopub = { version = \"[0-9]*\.[0-9]*\.[0-9]*\"/nanopub = { version = \"$new_version\"/" "$file"
        echo "🔼  Updated version in $file"
    else
        echo "⚠️ File not found: $file"
    fi
done
git cliff -o CHANGELOG.md --tag $new_version
git add Cargo.toml */Cargo.toml CHANGELOG.md
git commit -S -m "chore: Bump version to $new_version"
git push

echo ""
echo "  🏷️  Create and push tag"
echo ""
git tag -a v$new_version -m "v$new_version"
git push origin v$new_version

echo ""
echo "  🎉 Version $new_version released"

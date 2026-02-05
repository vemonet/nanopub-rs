#!/usr/bin/env bash
set -e

echo "Deprecated, now using cargo-release, checkout lib/docs/docs/contributing.md for more details"

# # Check if version argument is provided
# if [ "$#" -ne 1 ]; then
#     echo "Usage: $0 <new_version>"
#     exit 1
# fi
# new_version=$1

# echo ""
# echo "  🏔️ Update version in Cargo.toml"
# echo ""
# sed -i "" "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"
# sed -i "" "s/nanopub = { version = \"[0-9]*\.[0-9]*\.[0-9]*\"/nanopub = { version = \"$new_version\"/" "Cargo.toml"

# git cliff -o CHANGELOG.md --tag $new_version
# cargo update
# git add Cargo.toml Cargo.lock */Cargo.toml CHANGELOG.md
# git commit -m "chore: Bump version to $new_version"
# git push

# cd js
# npm run release
# cd ..

# echo ""
# echo "  🏷️  Create and push tag"
# echo ""
# git tag -a v$new_version -m "v$new_version"
# git push origin v$new_version

# echo ""
# echo "  🎉 Version $new_version released"


# NOTE: can't use cargo-release because it creates a CHANGELOG for each crate, and we only want one

# #!/usr/bin/env bash
# set -e

# # Release script using cargo-release
# # Usage: ./scripts/release.sh [patch|minor|major]
# # Or specify exact version: ./scripts/release.sh 0.2.0

# # Install cargo-release if not present:
# #   cargo install cargo-release

# # Check if version argument is provided
# if [ "$#" -ne 1 ]; then
#     echo "Usage: $0 <patch|minor|major|version>"
#     echo "Examples:"
#     echo "  $0 patch       # Bump patch version (0.1.5 -> 0.1.6)"
#     echo "  $0 minor       # Bump minor version (0.1.5 -> 0.2.0)"
#     echo "  $0 major       # Bump major version (0.1.5 -> 1.0.0)"
#     echo "  $0 0.2.0       # Set specific version"
#     exit 1
# fi

# version_arg=$1

# echo ""
# echo "  🚀 Starting release with cargo-release"
# echo ""

# # Dry run first to show what will happen
# echo "  📋 Dry run (showing what will happen):"
# cargo release "$version_arg" --no-confirm

# echo ""
# read -p "  ❓ Proceed with release? (y/N) " -n 1 -r
# echo ""

# if [[ $REPLY =~ ^[Yy]$ ]]; then
#     echo ""
#     echo "  🏔️ Executing release..."
#     cargo release "$version_arg" --execute --no-confirm

#     # Release JS package to npm
#     echo ""
#     echo "  📦 Building and publishing JS package to npm..."
#     cd js
#     npm run release
#     cd ..

#     echo ""
#     echo "  🎉 Release complete!"
# else
#     echo "  ❌ Release cancelled"
#     exit 1
# fi

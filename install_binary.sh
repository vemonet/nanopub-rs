#!/bin/bash
set -e

# Install nanopub binary for linux and MacOS
BINARY_NAME="np"

# Destination directory
DEST_DIR="/usr/local/bin"

BASE_BINARY_URL="http://github.com/vemonet/nanopub-rs/releases/latest/download/nanopub_x86_64_"
# Detect the platform (Linux or macOS)
case "$(uname)" in
    "Linux")
        PLATFORM="linux"
        BINARY_URL="${BASE_BINARY_URL}_linux_gnu"
        ;;
    "Darwin")
        PLATFORM="mac"
        BINARY_URL="${BASE_BINARY_URL}_apple"
        ;;
    *)
        echo "Unsupported platform: $(uname)" >&2
        exit 1
        ;;
esac

# Download the binary
curl -o "${BINARY_NAME}" "${BINARY_URL}"

# Make binary executable
chmod +x "${BINARY_NAME}"

# Move the binary to the destination directory
if [ -w "${DEST_DIR}" ]; then
    mv "${BINARY_NAME}" "${DEST_DIR}"
else
    echo "Admin permissions required to install to ${DEST_DIR}"
    sudo mv "${BINARY_NAME}" "${DEST_DIR}"
fi

echo "Installation completed. ${BINARY_NAME} installed to ${DEST_DIR}"

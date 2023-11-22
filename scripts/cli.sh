#!/usr/bin/env bash
set -e

cd cli

cargo run -- "$@"

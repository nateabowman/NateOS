#!/bin/bash

set -e

echo "Building NateOS..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed"
    exit 1
fi

# Build kernel
cargo build --release

echo "Build complete!"


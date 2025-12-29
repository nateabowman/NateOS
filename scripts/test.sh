#!/bin/bash

set -e

echo "Running NateOS tests..."

# Run unit tests
cargo test

# Run integration tests if available
if [ -d "tests" ]; then
    echo "Running integration tests..."
    # Add integration test commands here
fi

echo "Tests complete!"


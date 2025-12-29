#!/bin/bash

set -e

echo "Deploying NateOS..."

# Build release version
cargo build --release

# Create deployment directory
mkdir -p deploy

# Copy kernel
cp target/x86_64-nateos/release/nateos deploy/

# Create initramfs if needed
# TODO: Implement initramfs creation

echo "Deployment complete! Kernel is in deploy/"


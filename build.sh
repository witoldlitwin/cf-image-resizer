#!/bin/bash
set -e

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Install wasm-pack
echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install worker-build
echo "Installing worker-build..."
cargo install --git https://github.com/CathalMullan/workers-rs worker-build

# Build the worker
echo "Building worker..."
worker-build --release

# Copy the built worker to the output directory
echo "Copying built worker to output directory..."
mkdir -p dist
cp -r build/* dist/
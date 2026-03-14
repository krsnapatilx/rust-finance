#!/bin/bash
set -e

echo "Building RL Trading Bot in Release mode with native optimizations..."

export RUSTFLAGS="-C target-cpu=native"
cargo build --release --workspace

echo "Build complete. Binary located at target/release/daemon"

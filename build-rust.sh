#!/bin/bash

set -e

LIB_NAME="hashcore"
OUTPUT_DIR="target/universal/release"
LIB_FILENAME="lib${LIB_NAME}.dylib"

echo "--- Starting Rust Build for macOS (Universal) ---"

rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

echo "Building x86_64..."
cargo build --release --target x86_64-apple-darwin

echo "Building arm64..."
cargo build --release --target aarch64-apple-darwin

mkdir -p "$OUTPUT_DIR"

echo "Creating Universal Binary..."
lipo -create \
  target/x86_64-apple-darwin/release/$LIB_FILENAME \
  target/aarch64-apple-darwin/release/$LIB_FILENAME \
  -output "$OUTPUT_DIR/$LIB_FILENAME"

echo "Setting install name to @rpath..."
install_name_tool -id "@rpath/$LIB_FILENAME" "$OUTPUT_DIR/$LIB_FILENAME"

echo "--- Finished! The file is at $OUTPUT_DIR/ ---"
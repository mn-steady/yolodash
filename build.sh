#!/bin/bash
set -e

echo "Running trunk build --release..."
trunk build --release

echo "Running cargo build for wasm32-unknown-unknown target..."
cargo build --release --target wasm32-unknown-unknown

echo "Running wasm-bindgen..."
wasm-bindgen --out-dir dist/pkg --target web target/wasm32-unknown-unknown/release/yolodash.wasm

# Bundle JavaScript files with Webpack
echo "Bundling JavaScript files with webpack..."
npx webpack

echo "Copying static files..."
mkdir -p dist/static
cp -r static/* dist/static

echo "Build completed successfully!"

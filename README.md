# YoloDash
A Leptos-based Rust project with Shade API integration.

## Features
- YoloNode Theme
- Top bar with "YoloDash" and "Connect Wallet" button
- Keplr wallet connectivity

## Setup
1. Clone the repository and navigate to the directory.
2. Navigate to the project root directory
3. Run: ./build.sh
4. To test Locall: python3 -m http.server --bind 127.0.0.1
5. Push to github with pages setup

## Manual Steps if you dont want to use the build.sh
1. Clone the repository and navigate to the directory.
2. Run: trunk build --release 
3. Run: cargo build --release --target wasm32-unknown-unknown
4. Run: wasm-bindgen --out-dir dist/pkg --target web target/wasm32-unknown-unknown/release/yolodash.wasm
5. Run: mkdir -p dist/static
6. Run: cp static/* dist/static
7. To test Locall: python3 -m http.server --bind 127.0.0.1
8. Push to github with pages setup



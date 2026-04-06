#!/bin/bash
set -e

echo "Building SolScript WASM module..."
cd "$(dirname "$0")"

wasm-pack build --target web --release --out-dir ../frontend/src/wasm-pkg

echo "WASM build complete. Output in frontend/src/wasm-pkg/"

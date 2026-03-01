#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WWW_DIR="$ROOT_DIR/www"
PORT="${PORT:-8080}"

cargo build --release --target wasm32-unknown-unknown --manifest-path "$ROOT_DIR/Cargo.toml"
wasm-bindgen \
  --target web \
  --out-dir "$WWW_DIR/pkg" \
  --out-name n2t_wasm \
  "$ROOT_DIR/target/wasm32-unknown-unknown/release/n2t_wasm.wasm"

cd "$WWW_DIR"
echo "Serving at http://localhost:${PORT}"
python3 -m http.server "$PORT"

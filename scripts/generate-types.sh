#!/bin/bash
set -e

echo "Generating TypeScript types from Rust..."
cargo test --package shared

echo ""
echo "Types generated at frontend/src/types/bindings/"
ls -la frontend/src/types/bindings/

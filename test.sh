#!/bin/bash

set -e

echo "Running FastComments Rust SDK tests..."

# Run all tests
echo "Running tests..."
cargo test --verbose

echo ""
echo "Running doc tests..."
cargo test --doc

echo ""
echo "✓ All tests passed!"

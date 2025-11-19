#!/bin/bash

set -e

# Get version from Cargo.toml
VERSION=$(grep "^version = " Cargo.toml | head -1 | cut -d'"' -f2)

echo "Publishing FastComments Rust SDK v$VERSION..."

# Verify config.json matches
CONFIG_VERSION=$(jq -r '.packageVersion' config.json)
if [ "$CONFIG_VERSION" != "$VERSION" ]; then
  echo "Error: Version mismatch!"
  echo "config.json has version $CONFIG_VERSION but Cargo.toml has version $VERSION"
  echo "Run ./bump.sh $VERSION to fix this."
  exit 1
fi

# Run tests
echo "Running tests..."
cargo test

# Run checks
echo "Running cargo check..."
cargo check

# Build the package
echo "Building the package..."
cargo build --release

echo ""
echo "Publishing to crates.io..."
cargo publish

echo ""
echo "✓ Successfully published fastcomments-sdk v$VERSION to crates.io!"
echo ""
echo "Users can now install it with:"
echo "  cargo add fastcomments-sdk"

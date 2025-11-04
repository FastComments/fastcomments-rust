#!/bin/bash

# Remove previously generated code
rm -rvf ./client

# Try to download the OpenAPI spec from the running FastComments server
if wget -q http://localhost:3001/js/swagger.json -O ./openapi.json; then
    echo "Downloaded OpenAPI spec from running server"
    SPEC_FILE="./openapi.json"
else
    echo "Server not running, using local OpenAPI spec"
    SPEC_FILE="./openapi.yaml"
fi

# Generate the Rust client using openapi-generator
# Using npx to ensure consistent version with JS/Python SDKs
npx @openapitools/openapi-generator-cli generate \
    -i "$SPEC_FILE" \
    -g rust \
    -o ./client \
    --additional-properties=useSingleRequestParameter=true,disallowAdditionalPropertiesIfNotPresent=false \
    -c config.json \
    -t .openapi-generator/templates

echo "Generated Rust client in ./client"

# Fix import paths for nested module structure
echo "Fixing import paths..."
find ./client/src -type f -name "*.rs" -exec sed -i 's/use crate::{apis::ResponseContent, models}/use crate::client::{apis::ResponseContent, models}/g' {} \;
find ./client/src -type f -name "*.rs" -exec sed -i 's/use crate::models;/use crate::client::models;/g' {} \;
find ./client/src -type f -name "*.rs" -exec sed -i 's/crate::apis::urlencode/crate::client::apis::urlencode/g' {} \;

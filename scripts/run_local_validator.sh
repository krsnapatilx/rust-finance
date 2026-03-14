#!/bin/bash
set -e

echo "Starting local Solana validator..."

# Ensure solana-test-validator is installed
if ! command -v solana-test-validator &> /dev/null
then
    echo "solana-test-validator could not be found. Please install Solana CLI."
    exit 1
fi

solana-test-validator --reset --quiet

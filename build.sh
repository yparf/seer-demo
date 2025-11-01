#!/usr/bin/env bash
set -e

raw_version=$(solana --version 2>/dev/null)

if [ $? -ne 0 ]; then
    echo -e "\033[34m[SEER]\033[0m Solana CLI is not installed or not in PATH. Please install Solana CLI v3+:"
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/v3.0.1/install)\""
    exit 1
fi

version=$(echo "$raw_version" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')

major_version=$(echo "$version" | cut -d. -f1)

if [ "$major_version" -lt 3 ]; then
    echo -e "\033[34m[SEER]\033[0m Current Solana version detected: $version (INCOMPATIBLE)"
    echo -e "\033[34m[SEER]\033[0m Unfortunately, due to a bug in the Anza LLVM project fork, Seer only works with Solana CLI v3+. Please upgrade your Solana CLI:"
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/v3.0.1/install)\""
    exit 1
fi

echo -e "\033[34m[SEER]\033[0m Current Solana version detected: $version (OK)"

cargo build-sbf --debug --manifest-path programs/manager/Cargo.toml
cargo build-sbf --debug --manifest-path programs/nftminter/Cargo.toml
cargo build-sbf --debug --manifest-path programs/treasury/Cargo.toml

RUSTFLAGS="-A warnings" cargo build --package client
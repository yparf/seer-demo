#!/usr/bin/env bash
set -e

# solana programs
cargo build-sbf --debug --manifest-path programs/manager/Cargo.toml
cargo build-sbf --debug --manifest-path programs/nftminter/Cargo.toml
cargo build-sbf --debug --manifest-path programs/treasury/Cargo.toml

# client
RUSTFLAGS="-A warnings" cargo build --package client
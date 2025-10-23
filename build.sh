#!/usr/bin/env bash
set -e

# solana programs
AGAVE=../agave
cargo build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/manager/Cargo.toml
cargo build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/nftminter/Cargo.toml
cargo build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/treasury/Cargo.toml

# client
RUSTFLAGS="-A warnings" cargo build --package client
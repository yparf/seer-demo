#!/usr/bin/env bash
set -e

# solana programs
AGAVE=../agave
$AGAVE/target/debug/cargo-build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/manager/Cargo.toml
$AGAVE/target/debug/cargo-build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/nftminter/Cargo.toml
$AGAVE/target/debug/cargo-build-sbf --sbf-sdk $AGAVE/platform-tools-sdk/sbf --debug --manifest-path programs/treasury/Cargo.toml

# client
RUSTFLAGS="-A warnings" cargo build --package client
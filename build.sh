#!/usr/bin/env bash
set -e

cargo build-sbf --manifest-path programs/manager/Cargo.toml
cargo build-sbf --manifest-path programs/nftminter/Cargo.toml
cargo build-sbf --manifest-path programs/treasury/Cargo.toml

#!/usr/bin/env bash

# Treasury
TREASURY_PROGRAM_ID=$(solana program deploy target/deploy/treasury.so 2>&1 | grep "Program Id:" | awk '{print $3}') || TREASURY_PROGRAM_ID=""
export TREASURY_PROGRAM_ID
echo "Treasury Program ID: $TREASURY_PROGRAM_ID"

# Manager
MANAGER_PROGRAM_ID=$(solana program deploy target/deploy/manager.so 2>&1 | grep "Program Id:" | awk '{print $3}') || MANAGER_PROGRAM_ID=""
export MANAGER_PROGRAM_ID
echo "Manager Program ID: $MANAGER_PROGRAM_ID"

# NFT Minter
NFTMINTER_PROGRAM_ID=$(solana program deploy target/deploy/nftminter.so 2>&1 | grep "Program Id:" | awk '{print $3}') || NFTMINTER_PROGRAM_ID=""
export NFTMINTER_PROGRAM_ID
echo "NFT Minter Program ID: $NFTMINTER_PROGRAM_ID"

echo "Deployment complete. Environment variables are set."

# seer-demo

This is a mock Solana project created to demonstrate how **Seer** can be used for troubleshooting unexpected errors in Solana programs.

## Description

The project includes three Solana programs - **Manager**, **Treasury**, and **NFTMinter** - implementing a simple crowdfunding use case:

- **Admins** create campaigns through the Manager, setting up all configurations.
- **Users** contribute funds via the Treasury and receive an NFT badge from the NFTMinter as proof of contribution.

An intentional bug is introduced in the user flow to showcase how **Seer** can help identify and debug it.

## Crowdfunding Flow

```mermaid
sequenceDiagram
    autonumber
    participant User as User
    participant Manager as Manager Program
    participant Treasury as Treasury Program
    participant NFT as NFT Minter Program

    Note over User,NFT: Crowdfunding Flow with NFT Badge Minting

    User->>Manager: contribute_to_campaign(campaign_id, amount)
    Manager->>Treasury: deposit_funds(campaign_vault, amount)
    Treasury-->>Manager: *deposit success*
    Manager->>NFT: mint_contributor_badge(user, campaign_id)
    NFT-->>Manager: *badge minted*
    Manager-->>User: *contribution complete*
```


```mermaid
sequenceDiagram
    autonumber
    participant Admin as Admin / Deployer
    participant Manager as Manager Program
    participant Treasury as Treasury Program
    participant NFT as NFT Minter Program

    Note over Admin,NFT: Setup & Initialization Flow

    Admin->>Treasury: deploy_treasury_program()

    Admin->>NFT: deploy_nft_minter_program()

    Admin->>NFT: initialize_nft_config(admin_pubkey)

    Admin->>Manager: deploy_manager_program()

    Admin->>Manager: create_campaign(goal)
    Manager->>Treasury: create_vault(campaign_vault_pda)
    Manager-->>Admin: *campaign created with vault*
```

## How to Run
There is a client that runs the admin and user flow in a single transaction.
The admin flow instructions succeed, but the transaction **fails on the user flow** instructions - this failure is intentional for the demonstration purposes.

1. `./build.sh` - to build solana programs
2. `RUSTFLAGS="-A warnings" cargo build --package client` - to build a client
3. `target/debug/client` - to execute the client
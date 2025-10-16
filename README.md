# Crowdfunding

### Use Case Overview
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

### How to Execute
1. `solana-test-validator`
2. `./build.sh`
3. `. ./deploy.sh`
4. `cargo run --bin setup_flow.rs`
5. Copy the `Campaign Account` value from the output
6. `cargo run --bin user_flow.rs <campaign_account>`
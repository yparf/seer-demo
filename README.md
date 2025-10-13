# Crowdfunding

```mermaid
sequenceDiagram
    autonumber
    participant User as User
    participant Crowdfund as Crowdfund Program
    participant Treasury as Treasury Program
    participant NFT as NFT Minter Program

    Note over User,NFT: Crowdfunding Flow with NFT Badge Minting

    User->>Crowdfund: contribute_to_campaign(campaign_id, amount)
    Crowdfund->>Treasury: CPI - deposit_funds(campaign_vault, amount)
    Treasury-->>Crowdfund: deposit_success
    Crowdfund->>NFT: CPI - mint_contributor_badge(user, campaign_id)
    NFT-->>Crowdfund: badge_minted
    Crowdfund-->>User: contribution_complete
```
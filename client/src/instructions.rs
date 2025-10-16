use manager::Instruction as ManagerInstruction;
use nftminter::NftMinterInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn nftminter_initialize_config(nft_program_id: &Pubkey, payer: &Pubkey) -> Instruction {
    let (nft_config_pda, _) = Pubkey::find_program_address(&[b"nft_config"], nft_program_id);

    Instruction::new_with_borsh(
        *nft_program_id,
        &NftMinterInstruction::InitializeConfig,
        vec![
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new(nft_config_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
        ],
    )
}

pub fn manager_create_campaign(
    manager_program_id: &Pubkey,
    treasury_program_id: &Pubkey,
    payer: &Pubkey,
    campaign_account: &Pubkey,
) -> Instruction {
    let vault_pda =
        Pubkey::find_program_address(&[b"vault", campaign_account.as_ref()], treasury_program_id).0;

    Instruction::new_with_borsh(
        *manager_program_id,
        &ManagerInstruction::CreateCampaign {
            goal: 5_000_000_000,
        },
        vec![
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new(*campaign_account, true),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(*treasury_program_id, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
        ],
    )
}
pub fn manager_contribute(
    manager_program_id: &Pubkey,
    treasury_program_id: &Pubkey,
    nftminter_program_id: &Pubkey,
    contributor: &Pubkey,
    campaign_account: &Pubkey,
    mint_account: &Pubkey,
) -> Instruction {
    let vault_pda =
        Pubkey::find_program_address(&[b"vault", campaign_account.as_ref()], treasury_program_id).0;
    let nft_config_pda = Pubkey::find_program_address(&[b"nft_config"], nftminter_program_id).0;
    let mint_authority_pda =
        Pubkey::find_program_address(&[b"mint_authority"], nftminter_program_id).0;

    // Derive ATA for contributor and mint
    let recipient_token_account =
        spl_associated_token_account::get_associated_token_address(contributor, mint_account);

    Instruction::new_with_borsh(
        *manager_program_id,
        &ManagerInstruction::Contribute {
            amount: 1_000_000, // Example amount
        },
        vec![
            AccountMeta::new_readonly(*contributor, true),
            AccountMeta::new(*campaign_account, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(*treasury_program_id, false),
            AccountMeta::new_readonly(*nftminter_program_id, false),
            AccountMeta::new_readonly(nft_config_pda, false),
            AccountMeta::new(*mint_account, true),
            AccountMeta::new_readonly(mint_authority_pda, false),
            AccountMeta::new(recipient_token_account, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
        ],
    )
}

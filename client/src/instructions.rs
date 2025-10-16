use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(BorshSerialize)]
pub enum NftMinterInstruction {
    InitializeConfig,
}

#[derive(BorshSerialize)]
pub enum ManagerInstruction {
    CreateCampaign { goal: u64 },
}

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
    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault", campaign_account.as_ref()], treasury_program_id);

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

use solana_sdk::pubkey::Pubkey;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub manager_program_id: Pubkey,
    pub treasury_program_id: Pubkey,
    pub nftminter_program_id: Pubkey,
    pub rpc_url: String,
}

impl Config {
    pub fn load_config() -> Self {
        let manager_program_id = env::var("MANAGER_PROGRAM_ID")
            .expect("MANAGER_PROGRAM_ID environment variable not set")
            .parse()
            .expect("Invalid MANAGER_PROGRAM_ID");

        let treasury_program_id = env::var("TREASURY_PROGRAM_ID")
            .expect("TREASURY_PROGRAM_ID environment variable not set")
            .parse()
            .expect("Invalid TREASURY_PROGRAM_ID");

        let nftminter_program_id = env::var("NFTMINTER_PROGRAM_ID")
            .expect("NFTMINTER_PROGRAM_ID environment variable not set")
            .parse()
            .expect("Invalid NFTMINTER_PROGRAM_ID");

        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8899".to_string());

        Config {
            manager_program_id,
            treasury_program_id,
            nftminter_program_id,
            rpc_url,
        }
    }
}

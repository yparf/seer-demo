use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

pub fn connect_client() -> RpcClient {
    let rpc_url = String::from("http://localhost:8899");
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}

pub fn fund_payer(client: &RpcClient, payer: &Keypair, lamports: u64) {
    println!("Requesting airdrop...");
    let sig = client
        .request_airdrop(&payer.pubkey(), lamports)
        .expect("Failed to request airdrop");

    client
        .confirm_transaction(&sig)
        .expect("Failed to confirm airdrop");
    println!("Airdrop complete for {}", payer.pubkey());
}

pub fn deploy_program(_client: &RpcClient, _payer: &Keypair, _path: &str) -> Pubkey {
    Keypair::new().pubkey()
}

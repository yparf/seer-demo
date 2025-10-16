use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
};

pub fn connect_client(url: String) -> RpcClient {
    RpcClient::new_with_commitment(url, CommitmentConfig::confirmed())
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

pub fn new_funded_payer(client: &RpcClient) -> Keypair {
    let payer = Keypair::new();
    fund_payer(&client, &payer, 1_000_000_000);
    payer
}

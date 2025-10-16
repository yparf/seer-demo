use client::{config, helpers, instructions};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn run(
    client: &RpcClient,
    contributor: &Keypair,
    manager: &Pubkey,
    treasury: &Pubkey,
    nftminter: &Pubkey,
    campaign_account: &Pubkey,
) {
    let mint_account = Keypair::new();

    let contribute_ix = instructions::manager_contribute(
        &manager,
        &treasury,
        &nftminter,
        &contributor.pubkey(),
        &campaign_account,
        &mint_account.pubkey(),
    );

    let mut tx = Transaction::new_with_payer(&[contribute_ix], Some(&contributor.pubkey()));

    tx.sign(
        &[contributor, &mint_account],
        client.get_latest_blockhash().unwrap(),
    );

    println!("Sending user contribution transaction...");
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("User contribution complete: {}", sig),
        Err(err) => eprintln!("User contribution failed: {:?}", err),
    }
}

fn campaign_from_args() -> Pubkey {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <campaign_account>", args[0]);
        std::process::exit(1);
    }
    let campaign_account: Pubkey = args[1]
        .parse()
        .expect("Invalid campaign account public key");
    campaign_account
}

#[tokio::main]
async fn main() {
    let config = config::Config::load_config();
    let campaign_account = campaign_from_args();

    let client = helpers::connect_client(config.rpc_url);
    let contributor = helpers::new_funded_payer(&client).await;

    run(
        &client,
        &contributor,
        &config.manager_program_id,
        &config.treasury_program_id,
        &config.nftminter_program_id,
        &campaign_account,
    );
}

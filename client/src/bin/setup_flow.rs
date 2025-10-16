use client::{config, helpers, instructions};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn run(
    client: &RpcClient,
    payer: &Keypair,
    manager: &Pubkey,
    nftminter: &Pubkey,
    treasury: &Pubkey,
) {
    let campaign_account = Keypair::new();

    let nft_init_ix = instructions::nftminter_initialize_config(&nftminter, &payer.pubkey());
    let create_campaign_ix = instructions::manager_create_campaign(
        &manager,
        &treasury,
        &payer.pubkey(),
        &campaign_account.pubkey(),
    );

    let mut tx =
        Transaction::new_with_payer(&[nft_init_ix, create_campaign_ix], Some(&payer.pubkey()));

    tx.sign(
        &[payer, &campaign_account],
        client.get_latest_blockhash().unwrap(),
    );

    println!("Sending setup flow transaction...");
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            println!("Setup flow complete: {}", sig);
            println!("Campaign Account: {}", &campaign_account.pubkey());
        }
        Err(err) => eprintln!("Setup flow failed: {:?}", err),
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::load_config();
    let client = helpers::connect_client(config.rpc_url);

    let payer = helpers::new_funded_payer(&client).await;

    run(
        &client,
        &payer,
        &config.manager_program_id,
        &config.nftminter_program_id,
        &config.treasury_program_id,
    );
}

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod helpers;
mod instructions;

fn run_setup_flow(
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
        Ok(sig) => println!("Setup flow complete: {}", sig),
        Err(err) => eprintln!("Setup flow failed: {:?}", err),
    }
}

#[tokio::main]
async fn main() {
    let client = helpers::connect_client();

    let payer = Keypair::new();
    helpers::fund_payer(&client, &payer, 1_000_000_000);

    let manager = helpers::deploy_program(&client, &payer, "target/deploy/manager.so");
    let treasury = helpers::deploy_program(&client, &payer, "target/deploy/treasury.so");
    let nftminter = helpers::deploy_program(&client, &payer, "target/deploy/nftminter.so");
    run_setup_flow(&client, &payer, &manager, &treasury, &nftminter);
}

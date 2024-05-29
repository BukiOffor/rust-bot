
use std::str::FromStr;

use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_lang::AccountDeserialize;
use anchor_spl::token::TokenAccount;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use client::serialize::token::unpack_token_account;



fn main(){
    let connection_url = "https://api.mainnet-beta.solana.com";
    let provider = RpcClient::new_with_commitment(connection_url, CommitmentConfig::confirmed());

    let acc_pub_key = Pubkey::from_str("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2").unwrap_or_else(|e|{
        println!("Error: {}", e);
        Pubkey::default()
    
    });
    let data = provider.get_account_data(&acc_pub_key).unwrap();
    //let acc = TokenAccount::try_deserialize(&mut &data[..]).unwrap();
    //println!("{:?}", acc.amount);
    println!("{:?}", data)

}


use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer},
    utils::Ganache,

};

use hex::ToHex;

#[tokio::main]
async fn main() -> eyre::Result<()>{
 let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artifact";
 let ganache = Ganache::new().mnemonic(mnemonic).spawn();
 println!("HTTP Endpoint: {}", ganache.endpoint());

 let wallet: LocalWallet = ganache.keys()[0].clone().into();
 let first_address = wallet.address();
 println!(
    "wallet first address: {}",
    first_address.encode_hex::<String>()
 );

 let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

 let first_balance = provider.get_balance(first_address, None).await?;
 println!("wallet first address balance: {}", first_balance);

 // Query the blance of some random account
 let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
 let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
 let other_balance = provider.get_balance(other_address, None).await?;
 println!("Balance for address{}: {}", other_address_hex, first_balance);
 Ok(())
}


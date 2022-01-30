use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::time::Duration;
use web3::contract::Contract;
use web3::contract::Options;

/// Returns CID of uploaded file
///
/// # Arguments
///
/// *'filename' - A name of a file which should be uploaded to IPFS
async fn upload_to_ipsf(filename: &String) -> Option<String> {
    let contents = fs::read(filename).expect("Unable to read file");
    let data = Cursor::new(contents);

    let client = IpfsClient::default();

    match client.add(data).await {
        Ok(res) => {
            println!("Published CID: {}", res.hash);
            Some(res.hash)
        }
        Err(e) => {
            eprintln!("Error adding file: {}", e);
            None
        }
    }
}

/// Calls smart contract and pushes CID. In this function there are multiple 'wild unwraps' and
/// hardcoded values, due to low complexity, POC character and lack of specific requirements.
///
/// # Arguments
///
/// *'cid' - CID which sould be stored in hardcoded smart contract
async fn push_cid(cid: String) -> Result<web3::types::H256, Box<dyn Error>> {
    let transport = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(transport);

    //account
    let accounts = web3.eth().accounts().await?;

    let bytecode = include_str!("../build/StoreCID.bin");

    let contract = Contract::deploy(web3.eth(), include_bytes!("../build/StoreCID.abi"))?
        .confirmations(0)
        .poll_interval(Duration::from_secs(1))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;

    println!("contract: {}", contract.address());

    let mut transaction_options = web3::contract::Options::default();
    transaction_options.gas = Some(web3::types::U256::from_dec_str("3000000").unwrap());
    let result_hash = contract
        .call("addCID", cid, accounts[1], transaction_options)
        .await?;
    Ok(result_hash)
}

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String;
    if args.len() > 1 {
        filename = &args[1];
        println!("{}", filename);
    } else {
        eprintln!("Provide filename as cmd arg!");
        return;
    }

    match upload_to_ipsf(filename).await {
        Some(cid) => match push_cid(cid).await {
            Ok(hash) => println!("Result hash: {}", hash),
            Err(err) => println!("Error:{}", err),
        },
        None => {
            return;
        }
    }
}

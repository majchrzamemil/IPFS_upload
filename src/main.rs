use hex_literal::hex;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use std::env;
use std::fs;
use std::io::Cursor;

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
async fn push_cid(cid: String) {
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    // This field should be configurable, but due to lack of requirements left hardcoded
    let contract_address: web3::types::H160 =
        hex!("FadF67B8eB694977C4602A9bdda23E5F3Ab19EF1").into();

    // There is a need to build StoreCID contract before running
    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/StoreCID.abi"),
    )
    .unwrap();

    // This field should be configurable, but due to lack of requirements left hardcoded
    let address_from = hex!("B9ce73A5CaA58aE1720A5529FaC1e306fD5EC827").into();

    let mut transaction_options = web3::contract::Options::default();
    transaction_options.gas = Some(web3::types::U256::from_dec_str("3000000").unwrap());
    let result_hash = contract
        .call("addCID", cid, address_from, transaction_options)
        .await
        .unwrap();
    println!("Transaction hash: {}", result_hash);
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
        Some(cid) => push_cid(cid).await,
        None => {
            return;
        }
    }
}

use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use std::env;
use std::fs;
use std::io::Cursor;

use hex_literal::hex;

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
    let contents = fs::read(filename).expect("Unable to read file");
    let data = Cursor::new(contents);

    let client = IpfsClient::default();

    let cid: String;
    match client.add(data).await {
        Ok(res) => {
            println!("{}", res.hash);
            cid = res.hash;
        }
        Err(e) => {
            eprintln!("error adding file: {}", e);
            return;
        }
    }

    //WEB3 part
    //To pass via env: http transport, account, smart contract address(might set defaults)

    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    //let contract_abi = "build/StoreCID.abi";
    let contract_address: web3::types::H160 =
        hex!("FadF67B8eB694977C4602A9bdda23E5F3Ab19EF1").into();

    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/StoreCID.abi"),
    )
    .unwrap();
    let address_from = hex!("B9ce73A5CaA58aE1720A5529FaC1e306fD5EC827").into();

    println!("Publishing CID: {}", cid);

    let mut transaction_options = web3::contract::Options::default();
    transaction_options.gas = Some(web3::types::U256::from_dec_str("3000000").unwrap());
    let result_hash = contract
        .call(
            "addCID",
            cid,
            address_from,
            transaction_options,
        )
        .await
        .unwrap();
    println!("Transaction hash: {}", result_hash);
}

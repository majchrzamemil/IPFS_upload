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
        hex!("29cC4ecB8bbDBCF2d31bef45181c290952f2369B").into();

    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/StoreCID.abi"),
    )
    .unwrap();
    let address_from = hex!("4beE06f55345dEB02119E39f4c791018F3FAb8A6").into();

    println!("Publishing CID: {}", cid);

    let result_hash = contract
        .call(
            "addCID",
            cid,
            address_from,
            web3::contract::Options::default(),
        )
        .await
        .unwrap();
    println!("Transaction hash: {}", result_hash);
}

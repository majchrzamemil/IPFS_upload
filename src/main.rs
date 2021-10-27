use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use std::env;
use std::fs;
use std::io::Cursor;

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

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e),
    }
}

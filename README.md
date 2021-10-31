# IPFS_upload

Project has been run and tested using ganache.



There is no deploying smart contract inside code, for test it has been done via remix.

Some values are hardcoded but if needed can be made configurable.

# Build and Run

Before running project there is a need to compile smart contract :

'sol contracts/store_cid.sol --abi' in order to get abi file.

Run: cargo build && ./target/debug/ipfs_upload paht_to_file

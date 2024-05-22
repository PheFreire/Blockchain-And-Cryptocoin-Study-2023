extern crate bitcoin;
extern crate rust_bitcoin;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network::Testnet;

fn main() {
    let script_hex = "HEX_DO_SEU_SCRIPT_P2SH_HOSPEDADO_AQUI";

    let p2sh_address = Address::p2sh(&hex::decode(script_hex).unwrap(), Network::Testnet);

    println!("Endereço P2SH: {}", p2sh_address);
}
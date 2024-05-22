extern crate bitcoin;
extern crate rust_bitcoin;

use bitcoin::network::constants::Network;
use bitcoin::util::address::p2sh;
use bitcoin::util::key::PrivateKey;

fn main() {
    let private_key = PrivateKey::from_wif("SUA_CHAVE_PRIVADA_AQUI").unwrap();
    let public_key = private_key.public_key(Network::Testnet);

    let p2sh_script = p2sh::Address::p2pkh(&public_key, None, Network::Testnet).script_pubkey();

    println!("Endereço P2SH: {}", p2sh::Address::from_script(&p2sh_script, Network::Testnet).to_string());
    println!("Script Hex: {}", p2sh_script.to_hex());
}
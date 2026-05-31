use bitcoin_dojo::PrivateKey;
use bitcoin_dojo::utils::address_types::{AddressType, Network};

fn main() {
    let priv_k = PrivateKey::new();
    let pub_k = priv_k.public_key();
    println!(
        "My pubkey for testnet: {}",
        pub_k.address(AddressType::P2PKH, Network::Testnet)
    );
    println!("WIF format: {}", priv_k.to_wif(Network::Testnet, true))
}

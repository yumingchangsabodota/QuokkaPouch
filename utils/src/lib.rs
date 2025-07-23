
use bip39::Mnemonic;

pub fn get_random_mnemonic() -> String {
    let m = Mnemonic::generate(24).unwrap();
    m.to_string()
}



use sp_core::{sr25519, Pair, crypto::{Ss58AddressFormat,Ss58Codec}};
use sp_core::hexdisplay::HexDisplay;


#[derive(Debug)]
pub enum Crypto{
    Polkadot,
    Kusama,
    Litecoin,
}

#[derive(Debug)]
pub struct Wallet {
    pub mnemonic: String,
    pub password: String,
}

impl Wallet {

    pub fn get_ss58_address(&self, address_format: u16) -> String{
        // substrate chain uses address format 42
        // polkadot chain uses address format 0
        // kusama chain uses address format 2
        let ss58_address_format: Ss58AddressFormat = Ss58AddressFormat::custom(address_format);
        let pub_key = &self.get_public_key();
        pub_key.to_ss58check_with_version(ss58_address_format)
    }

    pub fn get_public_key(&self) -> sr25519::Public{
        let seed = self.get_seed();
        let key_pair = sr25519::Pair::from_seed(&seed);
        key_pair.public()
    }

    pub fn get_private_key(&self) -> String{
        let seed = self.get_seed();
        let seed_ref = &seed.as_ref();
        let private_key = HexDisplay::from(seed_ref);
        format!("{:?}", private_key)
    }

    fn get_seed(&self) -> [u8; 32]{
        let seed = sr25519::Pair::from_phrase(&self.mnemonic, Some(&self.password)).expect("Invalid seed slice;");
        return seed.1;
    }


}


#[cfg(test)]
mod tests {

    use utils::get_random_mnemonic;

    #[test]
    fn test_construct(){
        let p = "".to_string();
        let mnemonic = get_random_mnemonic();
        let w = super::Wallet{
            mnemonic: mnemonic,
            password: p,
        };
        println!("{:?}", w);
        println!("Private - {:?}", w.get_private_key());
        println!("Public - {:?}", w.get_public_key());
        println!("SS58 1 - {:?}", w.get_ss58_address(1));
        println!("Polkadot - {:?}", w.get_ss58_address(0));
        println!("Kusama - {:?}", w.get_ss58_address(2));
        assert!(true);
    }
   
}

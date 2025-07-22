


#[derive(Debug)]
pub enum Crypto{
    Polkadot,
    Kusama,
    Litecoin,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_enum(){
        println!("{:?}",crate::Crypto::Polkadot);
        assert!(true);
    }
   
}

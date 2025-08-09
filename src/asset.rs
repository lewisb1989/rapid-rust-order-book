#[derive(Clone, Default, Debug)]
pub enum Blockchain {
    #[default]
    Ethereum,
    Arbitrum
}

#[derive(Clone, Default)]
pub struct Asset {
    blockchain: Blockchain,
    symbol: String,
    decimals: u64
}

impl Asset {

    pub fn new(
        blockchain: Blockchain,
        symbol: &str,
        decimals: u64
    ) -> Self {
        Self {
            blockchain,
            symbol: symbol.to_owned(),
            decimals
        }
    }

    pub fn get_id(&self) -> String {
        format!("{:?}{}", self.blockchain, self.symbol)
    }
}
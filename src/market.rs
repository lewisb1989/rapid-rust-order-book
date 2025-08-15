#[derive(Debug, Clone)]
pub struct Market {
    symbol: String,
    max_price: u64,
    min_price: u64,
}

impl Market {
    /// Creates a new market
    pub fn new(symbol: &str, max_price: u64, min_price: u64) -> Self {
        Self {
            symbol: symbol.to_owned(),
            max_price,
            min_price,
        }
    }

    /// Get market symbol
    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }

    /// Get max permitted price
    pub fn get_max_price(&self) -> u64 {
        self.max_price
    }

    /// Get min permitted price
    pub fn get_min_price(&self) -> u64 {
        self.min_price
    }
}

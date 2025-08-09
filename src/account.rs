#[derive(Clone, Default)]
pub struct Account {
    id: u128,
    public_key: String,
    balance: u64,
    available: u64,
    locked: u64,
    asset: String
}

impl Account {

    pub fn new(
        id: u128,
        public_key: &str,
        balance: u64,
        available: u64,
        locked: u64,
        asset: &str
    ) -> Self {
        Self {
            id,
            public_key: public_key.to_owned(),
            balance,
            available,
            locked,
            asset: asset.to_owned()
        }
    }

    pub fn set_id(&mut self, id: u128) {
        self.id = id;
    }

    pub fn credit(&mut self, amount: u64) -> Result<bool, String> {
        Ok(true)
    }

    pub fn debit(&mut self, amount: u64) -> Result<bool, String> {
        Ok(true)
    }

    pub fn lock(&mut self, amount: u64) -> Result<bool, String> {
        Ok(true)
    }

    pub fn unlock(&mut self, amount: u64) -> Result<bool, String> {
        Ok(true)
    }
}
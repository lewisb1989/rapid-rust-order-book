use bincode::{Decode, Encode};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum OrderStatus {
    Open,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default, Encode, Decode)]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default, Encode, Decode)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    id: u64,
    price: u64,
    size: u64,
    remaining: u64,
    side: Side,
    status: OrderStatus,
    typ: OrderType,
}

impl Order {
    /// Creates a new order with specified values
    pub fn new(price: u64, size: u64, side: Side, status: OrderStatus, typ: OrderType) -> Self {
        Self {
            id: 0,
            price,
            remaining: size,
            side,
            size,
            status,
            typ,
        }
    }

    /// Returns the order id
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// Returns the order price
    pub fn get_price(&self) -> u64 {
        self.price
    }

    /// Returns the unfilled order size
    pub fn get_remaining(&self) -> u64 {
        self.remaining
    }

    /// Returns the order size
    pub fn get_size(&self) -> u64 {
        self.size
    }

    /// Sets the order price
    pub fn set_price(&mut self, price: u64) {
        self.price = price;
    }

    /// Sets the remaining size
    pub fn set_remaining(&mut self, remaining: u64) {
        self.remaining = remaining;
    }

    /// Sets the size
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Sets the side
    pub fn set_side(&mut self, side: Side) {
        self.side = side;
    }

    /// Sets the type
    pub fn set_type(&mut self, typ: OrderType) {
        self.typ = typ;
    }

    /// Sets the status
    pub fn set_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    /// Sets the id
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
}

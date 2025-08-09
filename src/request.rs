use bincode::{Decode, Encode};

use crate::order::{OrderType, Side};

#[derive(Debug)]
pub enum RequestType {
    SubmitOrder,
    CancelOrder
}

pub trait MarketRequest {
    fn get_symbol(&self) -> &String;
}

#[derive(Debug)]
pub struct SignedRequest {
    pub id: u128,
    pub request_type: RequestType,
    pub payload: Vec<u8>
}

#[derive(Debug, Encode, Decode, Default)]
pub struct SubmitOrderRequest {
    pub symbol: String,
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType
}

impl MarketRequest for SubmitOrderRequest {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

#[derive(Debug, Encode, Decode)]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub id: u64
}

impl MarketRequest for CancelOrderRequest {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}
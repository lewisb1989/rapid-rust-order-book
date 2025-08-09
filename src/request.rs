use bincode::{Decode, Encode};

use crate::{asset::Blockchain, order::{OrderType, Side}};

#[derive(Debug)]
pub enum RequestType {
    SubmitOrder,
    CancelOrder,
    CreditAccount,
    DebitAccount,
    LockAccount,
    UnlockAccount
}

pub trait MarketRequest {
    fn get_symbol(&self) -> &String;
}

pub trait AccountRequest {
    fn get_id(&self) -> u128;
}

#[derive(Debug)]
pub struct WrappedRequest {
    pub id: u128,
    pub request_type: RequestType,
    pub payload: Vec<u8>,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

pub trait SignedRequest {
    fn get_signature(&self) -> &String;
    fn get_public_key(&self) -> &String;
    fn get_nonce(&self) -> u128;
}

#[derive(Debug, Encode, Decode, Default)]
pub struct SubmitOrderRequest {
    pub symbol: String,
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl SignedRequest for SubmitOrderRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

impl MarketRequest for SubmitOrderRequest {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

#[derive(Debug, Encode, Decode)]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub id: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl SignedRequest for CancelOrderRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

impl MarketRequest for CancelOrderRequest {
    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

#[derive(Debug)]
pub struct ListMarketRequest {
    pub symbol: String, 
    pub max_price: u64, 
    pub min_price: u64, 
    pub settlement_asset: String,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl SignedRequest for ListMarketRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

#[derive(Debug)]
pub struct ListAssetRequest {
    pub blockchain: Blockchain,
    pub symbol: String,
    pub decimals: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl SignedRequest for ListAssetRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

#[derive(Debug, Encode, Decode)]
pub struct CreditAccountRequest {
    pub account_id: u128,
    pub asset: String,
    pub amount: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl AccountRequest for CreditAccountRequest {
    fn get_id(&self) -> u128 {
        self.account_id
    }
}

impl SignedRequest for CreditAccountRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

#[derive(Debug, Encode, Decode)]
pub struct DebitAccountRequest {
    pub account_id: u128,
    pub asset: String,
    pub amount: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl AccountRequest for DebitAccountRequest {
    fn get_id(&self) -> u128 {
        self.account_id
    }
}

impl SignedRequest for DebitAccountRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

#[derive(Debug, Encode, Decode)]
pub struct LockAccountRequest {
    pub account_id: u128,
    pub asset: String,
    pub amount: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl AccountRequest for LockAccountRequest {
    fn get_id(&self) -> u128 {
        self.account_id
    }
}

impl SignedRequest for LockAccountRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}

#[derive(Debug, Encode, Decode)]
pub struct UnlockAccountRequest {
    pub account_id: u128,
    pub asset: String,
    pub amount: u64,
    pub signature: String,
    pub nonce: u128,
    pub public_key: String
}

impl AccountRequest for UnlockAccountRequest {
    fn get_id(&self) -> u128 {
        self.account_id
    }
}

impl SignedRequest for UnlockAccountRequest {
    fn get_signature(&self) -> &String {
        &self.signature
    }
    fn get_public_key(&self) -> &String {
        &self.public_key
    }
    fn get_nonce(&self) -> u128 {
        self.nonce
    }    
}
use std::{
    collections::HashMap, 
    sync::atomic::AtomicPtr
};

use crate::{
    account::Account, asset::Asset, market::Market, order::Order, order_book::{BestQuote, OrderBook}, request::RequestType
};

type RequestResults = HashMap<u128, Option<Result<(RequestType, Vec<u8>), String>>>;

pub struct State {
    markets: AtomicPtr<HashMap<String, Market>>,
    order_books: AtomicPtr<HashMap<String, OrderBook>>,
    assets: AtomicPtr<HashMap<String, Asset>>,
    accounts: AtomicPtr<HashMap<String, HashMap<u128, Account>>>,
    market_request_results: AtomicPtr<RequestResults>,
    account_request_results: AtomicPtr<RequestResults>
    // TODO - cache the user's orders
    // TODO - add positions
    // TODO - cache the user's positions
}

impl State {

    /// Create instance of internal state
    pub fn new() -> Self {
        // pre-allocate request results
        let mut market_request_results = HashMap::new();
        let mut account_request_results = HashMap::new();
        for i in 0..10_000_000 {
            market_request_results.insert(i+1, None);
            account_request_results.insert(i+1, None);
        }
        println!("pre-allocated memory!");
        Self {
            markets: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            order_books: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            accounts: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            assets: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            market_request_results: AtomicPtr::new(Box::into_raw(Box::new(market_request_results))),
            account_request_results: AtomicPtr::new(Box::into_raw(Box::new(account_request_results)))
        }
    }

    /// Get mutable reference to assets
    #[allow(clippy::mut_from_ref)]
    pub fn get_assets(&self) -> &mut HashMap<String, Asset> {
        unsafe {
            (*self.assets.as_ptr()).as_mut().unwrap()
        }
    }

    /// Get mutable reference to accounts
    #[allow(clippy::mut_from_ref)]
    pub fn get_accounts(&self) -> &mut HashMap<String, HashMap<u128, Account>> {
        unsafe {
            (*self.accounts.as_ptr()).as_mut().unwrap()
        }
    }

    /// Get mutable reference to order books
    #[allow(clippy::mut_from_ref)]
    pub fn get_order_books(&self) -> &mut HashMap<String, OrderBook> {
        unsafe {
            (*self.order_books.as_ptr()).as_mut().unwrap()
        }
    }

    /// Get mutable reference to market request results
    #[allow(clippy::mut_from_ref)]
    pub fn get_market_request_results(&self) -> &mut RequestResults {
        unsafe {
            (*self.market_request_results.as_ptr()).as_mut().unwrap()
        }
    }
    
    /// Get mutable reference to account request results
    #[allow(clippy::mut_from_ref)]
    pub fn get_account_request_results(&self) -> &mut RequestResults {
        unsafe {
            (*self.account_request_results.as_ptr()).as_mut().unwrap()
        }
    }

    /// Get mutable reference to markets
    #[allow(clippy::mut_from_ref)]
    pub fn get_markets(&self) -> &mut HashMap<String, Market> {
        unsafe {
            (*self.markets.as_ptr()).as_mut().unwrap()
        }
    }

    /// Get mutable reference to account for asset and ID
    pub fn get_account_by_asset_and_id(&self, asset: &str, id: u128) -> Result<&mut Account, String> {
        match self.get_accounts().get_mut(asset) {
            Some(accounts_by_id) => {
                match accounts_by_id.get_mut(&id) {
                    Some(account) => Ok(account),
                    None => Err("account not found".to_string())
                }
            },
            None => Err("account not found".to_string())
        }
    }

    /// Get mutable reference to order book for given symbol
    #[allow(clippy::mut_from_ref)]
    pub fn get_order_book_by_symbol(&self, symbol: &String) -> Result<&mut OrderBook, String> {
        match self.get_order_books().get_mut(symbol) {
            Some(order_book) => Ok(order_book),
            None => Err("order book not found".to_string())
        }
    }

    /// Save market request result for given request ID
    #[allow(clippy::mut_from_ref)]
    pub fn save_market_request_result(&self, request_id: u128, result: Result<(RequestType, Vec<u8>), String>) {
        *self.get_market_request_results().get_mut(&request_id).unwrap() = Some(result);
    }

    /// Save account request result for given request ID
    #[allow(clippy::mut_from_ref)]
    pub fn save_account_request_result(&self, request_id: u128, result: Result<(RequestType, Vec<u8>), String>) {
        *self.get_account_request_results().get_mut(&request_id).unwrap() = Some(result);
    }

    /// Get the best bid and best ask for the specified market
    pub fn get_best_quote(&self, symbol: &String) -> Result<BestQuote, String> {
        let order_book = self.get_order_book_by_symbol(symbol)?;
        let best_bid_price = order_book.get_best_bid_price();
        let best_bid_size = order_book.get_best_bid_size();
        let best_ask_price = order_book.get_best_ask_price();
        let best_ask_size = order_book.get_best_ask_size();
        Ok(BestQuote {
            best_bid_price,
            best_bid_size,
            best_ask_price,
            best_ask_size
        })
    }

    /// Get all orders for the specified market
    pub fn get_orders_by_symbol(&self, symbol: &String) -> Result<Vec<Order>, String> {
        let order_book = self.get_order_book_by_symbol(symbol)?;
        let bids = order_book.get_bids();
        let asks = order_book.get_asks();
        let mut orders = Vec::new();
        for bid in bids {
            for order in bid.get_orders() {
                if order.get_size() == 0 {
                    break;
                }
                orders.push(*order);
            }
        }
        for ask in asks {
            for order in ask.get_orders() {
                if order.get_size() == 0 {
                    break;
                }
                orders.push(*order);
            }
        }
        Ok(orders)
    }
}
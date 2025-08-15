use std::{collections::HashMap, sync::atomic::AtomicPtr};

use crate::{market::Market, order_book::OrderBook, request::RequestType};

type RequestResults = HashMap<u128, Option<Result<(RequestType, Vec<u8>), String>>>;

pub struct State {
    markets: AtomicPtr<HashMap<String, Market>>,
    order_books: AtomicPtr<HashMap<String, OrderBook>>,
    request_results: AtomicPtr<RequestResults>,
}

impl State {
    /// Create instance of internal state
    pub fn new() -> Self {
        let mut request_results = HashMap::new();
        for i in 0..10_000_000 {
            request_results.insert(i + 1, None);
        }
        println!("pre-allocated memory!");
        Self {
            markets: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            order_books: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            request_results: AtomicPtr::new(Box::into_raw(Box::new(request_results))),
        }
    }

    /// Get mutable reference to order books
    #[allow(clippy::mut_from_ref)]
    pub fn get_order_books(&self) -> &mut HashMap<String, OrderBook> {
        unsafe { (*self.order_books.as_ptr()).as_mut().unwrap() }
    }

    /// Get mutable reference to order book for given symbol
    #[allow(clippy::mut_from_ref)]
    pub fn get_order_book_by_symbol(&self, symbol: &String) -> Result<&mut OrderBook, String> {
        match self.get_order_books().get_mut(symbol) {
            Some(order_book) => Ok(order_book),
            None => Err("order book not found".to_string()),
        }
    }

    /// Get mutable reference to request results
    #[allow(clippy::mut_from_ref)]
    pub fn get_request_results(&self) -> &mut RequestResults {
        unsafe { (*self.request_results.as_ptr()).as_mut().unwrap() }
    }

    /// Get mutable reference to markets
    #[allow(clippy::mut_from_ref)]
    pub fn get_markets(&self) -> &mut HashMap<String, Market> {
        unsafe { (*self.markets.as_ptr()).as_mut().unwrap() }
    }

    /// Save request result for given request ID
    #[allow(clippy::mut_from_ref)]
    pub fn save_request_result(
        &self,
        request_id: u128,
        result: Result<(RequestType, Vec<u8>), String>,
    ) {
        *self.get_request_results().get_mut(&request_id).unwrap() = Some(result);
    }
}

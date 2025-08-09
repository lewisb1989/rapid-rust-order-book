use std::{
    collections::HashMap, 
    sync::{
        mpsc::{
            self, 
            Sender
        }, 
        Arc, 
        RwLock 
    }, 
    thread
};
use bincode::{
    Decode, 
    Encode
};

use crate::{
    market::Market, 
    order::Order, 
    order_book::{
        BestQuote, 
        OrderBook
    }, 
    request::{
        CancelOrderRequest, 
        MarketRequest, 
        RequestType, 
        SignedRequest, 
        SubmitOrderRequest
    }, state::State
};

pub struct Exchange {
    channels: Vec<Sender<SignedRequest>>,
    channel_by_symbol: HashMap<String, usize>,
    last_req_id: u128,
    markets_lock: Arc<RwLock<bool>>,
    state: Arc<State>
}

impl Exchange {
    
    /// Creates a new exchange
    pub fn new() -> Self {
        println!("initializing the exchange...");
        let mut channels: Vec<Sender<SignedRequest>> = Vec::new();
        let state = Arc::new(State::new());
        let markets_lock = Arc::new(RwLock::new(true));
        Self::setup_worker_threads(&mut channels, &state, &markets_lock);
        Self {
            state,
            channels,
            channel_by_symbol: HashMap::new(),
            last_req_id: 0,
            markets_lock
        }
    }

    /// Setup worker threads used to process incoming requests
    fn setup_worker_threads(
        channels: &mut Vec<Sender<SignedRequest>>,
        state: &Arc<State>,
        markets_lock: &Arc<RwLock<bool>>
    ) {
        // divide total CPUs by 2 to get physical cores
        let core_ids = core_affinity::get_core_ids().unwrap();
        // for each core, spawn a worker thread
        for core_id in core_ids {
            // create new channel used to send requests to the worker thread
            let (tx, rx) = mpsc::channel();
            channels.push(tx);
            // clone the state so each thread can hold its own reference
            let state_clone = state.clone();
            let markets_lock_clone = markets_lock.clone();
            thread::spawn(move || {
                core_affinity::set_for_current(core_id);
                // process incoming requests
                for request in rx {
                    let lock = markets_lock_clone.read();
                    Self::handle_signed_request(&state_clone, request);
                    drop(lock);
                }
            });
            println!("created worker thread {}", core_id.id);
        }
    }

    /// Handle a signed request when it is received by a worker thread
    fn handle_signed_request(
        state: &Arc<State>,
        request: SignedRequest
    ) {
        let request_id = request.id;
        match request.request_type {
            RequestType::SubmitOrder => {
                let mut request: SubmitOrderRequest = Self::decode_payload(request.payload);
                let result = Self::handle_submit_order(state, &mut request);
                state.save_request_result(request_id, result);
            },
            RequestType::CancelOrder => {
                let mut request: CancelOrderRequest = Self::decode_payload(request.payload);
                let result = Self::handle_cancel_order(state, &mut request);
                state.save_request_result(request_id, result);
            }
        }
    }

    /// Decode binary payload into request object
    fn decode_payload<T: Decode<()>>(payload: Vec<u8>) -> T {
        let (request, _): (T, usize) = bincode::decode_from_slice(
            &payload, 
            bincode::config::standard()
        ).unwrap();
        request
    }

    /// Handle new order submission requests
    fn handle_submit_order(
        state: &Arc<State>,
        request: &mut SubmitOrderRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let order_book = state.get_order_book_by_symbol(request.get_symbol())?;
        let id = order_book.submit_order(request)?;
        let response = bincode::encode_to_vec(id, bincode::config::standard()).unwrap();
        Ok((RequestType::SubmitOrder, response))
    }

    /// Handle order cancellation requests
    fn handle_cancel_order(
        state: &Arc<State>,
        request: &mut CancelOrderRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let order_book = state.get_order_book_by_symbol(request.get_symbol())?;
        let result = order_book.cancel_order(request)?;
        let response = bincode::encode_to_vec(result, bincode::config::standard()).unwrap();
        Ok((RequestType::CancelOrder, response))
    }

    /// Get the channel ID for the given symbol
    /// 
    /// Each market submits requests exclusively to a single channel (aka worker thread)
    /// 
    /// Multiple markets can be processed by a single worker thread, if there are more
    /// listed markets than physical cores
    fn get_channel_id(&self, symbol: &str) -> Result<usize, String> {
        match self.channel_by_symbol.get(symbol) {
            Some(id) => Ok(*id),
            None => Err("market not found".to_string())
        }
    }

    /// Get the channel transmitter for given channel ID
    fn get_channel(&self, id: usize) -> Result<&Sender<SignedRequest>, String> {
        match self.channels.get(id) {
            Some(sender) => Ok(sender),
            None => Err("market not found".to_string())
        }
    }

    /// Encode binary payload from request object
    fn build_payload<T: Encode>(&self, request: T) -> Result<Vec<u8>, String> {
        match bincode::encode_to_vec(request, bincode::config::standard()) {
            Ok(res) => Ok(res),
            Err(err) => Err(format!("cannot build payload: {}", err))
        }
    }

    /// Submit incoming requests for async processing, by sending them to the 
    /// channel allocated to the specified market
    fn handle_request<T: Encode + MarketRequest>(&mut self, request: T, request_type: RequestType) -> Result<u128, String> {
        if self.last_req_id == (self.state.get_request_results().len() as u128) - 1 {
            self.last_req_id = 0;
        }
        self.last_req_id += 1;
        let channel_id = self.get_channel_id(request.get_symbol())?;
        let sender = self.get_channel(channel_id)?;
        let payload = self.build_payload(request)?;
        let result = sender.send(SignedRequest { 
            id: self.last_req_id,
            request_type,
            payload
        });
        match result {
            Ok(_) => Ok(self.last_req_id),
            Err(err) => Err(err.to_string())
        }
    }

    /// Get all markets
    pub fn get_markets(&self) -> Vec<Market> {
        self.state.get_markets().values().cloned().collect()
    }

    /// List a new market
    /// 
    /// Note: since this function modifies the size of the markets hashmap, it must
    /// lock the internal state until the market is added to prevent the matching
    /// engine from modifying memory addresses that have been de-allocated
    pub fn list_market(&mut self, 
        symbol: &str, 
        max_price: u64, 
        min_price: u64, 
    ) -> Result<&Market, String> {
        let markets = self.state.get_markets();
        if markets.get(symbol).is_none() {
            let market = Market::new(symbol, max_price, min_price);
            let channel_id = markets.len() % self.channels.len();
            self.channel_by_symbol.insert(symbol.to_string(), channel_id);
            markets.insert(symbol.to_string(), market.clone());
            // we need exclusive write access here as this operation allocates heap memory
            let lock = self.markets_lock.write();
            // create new order book
            self.state.get_order_books().insert(symbol.to_string(), OrderBook::new(market));
            // drop the lock now that we're done
            drop(lock);
            Ok(markets.get(symbol).unwrap())
        } else {
            Err("market already exists".to_string())
        }
    }

    /// Submit a new order
    pub fn submit_order(&mut self, request: SubmitOrderRequest) -> Result<u128, String> {
        self.handle_request(request, RequestType::SubmitOrder)
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, request: CancelOrderRequest) -> Result<u128, String> {
        self.handle_request(request, RequestType::CancelOrder)
    }

    /// Get the request results in binary format for a list of known request IDs
    /// 
    /// Note: this function will block until all requests have either completed 
    /// successfully or otherwise failed
    pub fn get_results(&self, request_ids: Vec<u128>) -> Vec<&Result<(RequestType, Vec<u8>), String>> {
        let mut results = Vec::new();
        for id in request_ids {
            loop {
                if let Some(Some(result)) = self.state.get_request_results().get(&id) {
                    results.push(result);
                    break;
                }
            }
        }
        results
    }

    /// Get the best bid and best ask for the specified market
    pub fn get_best_quote(&self, symbol: &String) -> Result<BestQuote, String> {
        let order_book = self.state.get_order_book_by_symbol(symbol)?;
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
        let order_book = self.state.get_order_book_by_symbol(symbol)?;
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
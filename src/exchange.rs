use std::{
    collections::HashMap, 
    sync::{
        mpsc::{
            self, 
            Receiver, 
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
use core_affinity::CoreId;

use crate::{
    account::Account, 
    asset::Asset, 
    market::Market, 
    order::Order, 
    order_book::{
        BestQuote, 
        OrderBook
    }, request::{
        AccountRequest, 
        CancelOrderRequest, 
        CreditAccountRequest, 
        DebitAccountRequest, 
        ListAssetRequest, 
        ListMarketRequest, 
        LockAccountRequest, 
        MarketRequest, 
        RequestType, 
        SignedRequest, 
        SubmitOrderRequest, 
        UnlockAccountRequest, 
        WrappedRequest
    }, 
    state::State
};

struct ExchangeStats {

}

pub struct Exchange {
    market_channels: Vec<Sender<WrappedRequest>>,
    account_channels: Vec<Sender<WrappedRequest>>,
    market_channel_by_symbol: HashMap<String, usize>,
    account_channel_by_id: HashMap<u128, usize>,
    market_req_id: u128,
    account_req_id: u128,
    markets_lock: Arc<RwLock<bool>>,
    accounts_lock: Arc<RwLock<bool>>,
    state: Arc<State>
}

impl Exchange {
    
    /// Creates a new exchange
    pub fn new() -> Self {
        println!("initializing the exchange...");
        let mut market_channels: Vec<Sender<WrappedRequest>> = Vec::new();
        let mut account_channels: Vec<Sender<WrappedRequest>> = Vec::new();
        let state = Arc::new(State::new());
        let markets_lock = Arc::new(RwLock::new(true));
        let accounts_lock = Arc::new(RwLock::new(true));
        Self::setup_worker_threads(&mut market_channels, &mut account_channels, &state, &markets_lock, &accounts_lock);
        Self {
            state,
            market_channels,
            account_channels,
            market_channel_by_symbol: HashMap::new(),
            account_channel_by_id: HashMap::new(),
            market_req_id: 0,
            account_req_id: 0,
            markets_lock,
            accounts_lock
        }
    }

    fn spawn_worker_thread(
        core_id: CoreId,
        rx: Receiver<WrappedRequest>,
        lock: Arc<RwLock<bool>>,
        state: Arc<State>
    ) {
        thread::spawn(move || {
            // pin this thread to current core
            core_affinity::set_for_current(core_id);
            // process incoming requests
            for request in rx {
                let lock = lock.read();
                Self::handle_signed_request(&state, request);
                drop(lock);
            }
        });
        println!("created worker thread {}", core_id.id);
    }

    /// Setup worker threads used to process incoming requests
    fn setup_worker_threads(
        market_channels: &mut Vec<Sender<WrappedRequest>>,
        account_channels: &mut Vec<Sender<WrappedRequest>>,
        state: &Arc<State>,
        markets_lock: &Arc<RwLock<bool>>,
        accounts_lock: &Arc<RwLock<bool>>
    ) {
        // get CPUs
        let core_ids = core_affinity::get_core_ids().unwrap();
        // for each core, spawn a worker thread
        for core_id in core_ids {
            // create channel used to send requests to market worker thread
            let (market_tx, market_rx) = mpsc::channel();
            market_channels.push(market_tx);
            // create channel used to send requests to account worker thread
            let (account_tx, account_rx) = mpsc::channel();
            account_channels.push(account_tx);
            // clone the state so each market worker thread can hold its own reference
            let state_clone = state.clone();
            // clone the markets lock
            let markets_lock_clone = markets_lock.clone();
            // spawn worker thread for market requests
            Self::spawn_worker_thread(core_id, market_rx, markets_lock_clone, state_clone);
            // clone the state again so that the accounts worker thread can also hold a reference to it
            let state_clone = state.clone();
            // clone the accounts lock
            let accounts_lock_clone = accounts_lock.clone();
            // spawn worker thread for account requests
            Self::spawn_worker_thread(core_id, account_rx, accounts_lock_clone, state_clone);
        }
    }

    /// Handle a signed request when it is received by a worker thread
    fn handle_signed_request(
        state: &Arc<State>,
        request: WrappedRequest
    ) {
        let request_id = request.id;
        match request.request_type {
            RequestType::SubmitOrder => {
                let mut request: SubmitOrderRequest = Self::decode_payload(request.payload);
                let result = Self::handle_submit_order(state, &mut request);
                state.save_market_request_result(request_id, result);
            },
            RequestType::CancelOrder => {
                let mut request: CancelOrderRequest = Self::decode_payload(request.payload);
                let result = Self::handle_cancel_order(state, &mut request);
                state.save_market_request_result(request_id, result);
            },
            RequestType::CreditAccount => {
                let mut request: CreditAccountRequest = Self::decode_payload(request.payload);
                let result = Self::handle_credit_account(state, &mut request);
                state.save_account_request_result(request_id, result);
            },
            RequestType::DebitAccount => {
                let mut request: DebitAccountRequest = Self::decode_payload(request.payload);
                let result = Self::handle_debit_account(state, &mut request);
                state.save_account_request_result(request_id, result);
            },
            RequestType::LockAccount => {
                let mut request: LockAccountRequest = Self::decode_payload(request.payload);
                let result = Self::handle_lock_account(state, &mut request);
                state.save_account_request_result(request_id, result);
            },
            RequestType::UnlockAccount => {
                let mut request: UnlockAccountRequest = Self::decode_payload(request.payload);
                let result = Self::handle_unlock_account(state, &mut request);
                state.save_account_request_result(request_id, result);
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

    /// Handle credit account requests
    fn handle_credit_account(
        state: &Arc<State>,
        request: &mut CreditAccountRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let account = state.get_account_by_asset_and_id(&request.asset, request.get_id())?;
        let result = account.credit(request.amount)?;
        let response = bincode::encode_to_vec(result, bincode::config::standard()).unwrap();
        Ok((RequestType::CreditAccount, response))
    }

    /// Handle debit account requests
    fn handle_debit_account(
        state: &Arc<State>,
        request: &mut DebitAccountRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let account = state.get_account_by_asset_and_id(&request.asset, request.get_id())?;
        let result = account.debit(request.amount)?;
        let response = bincode::encode_to_vec(result, bincode::config::standard()).unwrap();
        Ok((RequestType::DebitAccount, response))
    }

    /// Handle lock account requests
    fn handle_lock_account(
        state: &Arc<State>,
        request: &mut LockAccountRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let account = state.get_account_by_asset_and_id(&request.asset, request.get_id())?;
        let result = account.lock(request.amount)?;
        let response = bincode::encode_to_vec(result, bincode::config::standard()).unwrap();
        Ok((RequestType::LockAccount, response))
    }

    /// Handle unlock account requests
    fn handle_unlock_account(
        state: &Arc<State>,
        request: &mut UnlockAccountRequest
    ) -> Result<(RequestType, Vec<u8>), String> {
        let account = state.get_account_by_asset_and_id(&request.asset, request.get_id())?;
        let result = account.unlock(request.amount)?;
        let response = bincode::encode_to_vec(result, bincode::config::standard()).unwrap();
        Ok((RequestType::UnlockAccount, response))
    }

    /// Get the channel ID for the given symbol
    /// 
    /// Each market submits requests exclusively to a single channel (aka worker thread)
    /// 
    /// Multiple markets can be processed by a single worker thread, if there are more
    /// listed markets than physical cores
    fn get_market_channel_id(&self, symbol: &str) -> Result<usize, String> {
        match self.market_channel_by_symbol.get(symbol) {
            Some(id) => Ok(*id),
            None => Err("market not found".to_string())
        }
    }

    /// Get the channel transmitter for given channel ID
    fn get_market_channel(&self, id: usize) -> Result<&Sender<WrappedRequest>, String> {
        match self.market_channels.get(id) {
            Some(sender) => Ok(sender),
            None => Err("market not found".to_string())
        }
    }

    /// Get the channel ID for the given account ID
    /// 
    /// Each account submits requests exclusively to a single channel (aka worker thread)
    /// 
    /// Multiple accounts can be processed by a single worker thread, if there are more
    /// listed accounts than physical cores
    fn get_account_channel_id(&self, id: u128) -> Result<usize, String> {
        match self.account_channel_by_id.get(&id) {
            Some(id) => Ok(*id),
            None => Err("account not found".to_string())
        }
    }

    /// Get the channel transmitter for given channel ID
    fn get_account_channel(&self, id: usize) -> Result<&Sender<WrappedRequest>, String> {
        match self.account_channels.get(id) {
            Some(sender) => Ok(sender),
            None => Err("account not found".to_string())
        }
    }

    /// Encode binary payload from request object
    fn build_payload<T: Encode>(&self, request: &T) -> Result<Vec<u8>, String> {
        match bincode::encode_to_vec(request, bincode::config::standard()) {
            Ok(res) => Ok(res),
            Err(err) => Err(format!("cannot build payload: {}", err))
        }
    }

    /// Submit incoming market requests for async processing, 
    /// by sending them to the allocated market channel
    fn handle_market_request<T: Encode + MarketRequest + SignedRequest>(
        &mut self, 
        request: T, 
        request_type: RequestType
    ) -> Result<u128, String> {
        if self.market_req_id == (self.state.get_market_request_results().len() as u128) - 1 {
            self.market_req_id = 0;
        }
        self.market_req_id += 1;
        let channel_id = self.get_market_channel_id(request.get_symbol())?;
        let sender = self.get_market_channel(channel_id)?;
        let payload = self.build_payload(&request)?;
        let result = sender.send(WrappedRequest { 
            id: self.market_req_id,
            request_type,
            payload,
            signature: request.get_signature().to_string(),
            nonce: request.get_nonce(),
            public_key: request.get_public_key().to_string()
        });
        match result {
            Ok(_) => Ok(self.market_req_id),
            Err(err) => Err(err.to_string())
        }
    }

    /// Submit incoming account requests for async processing, 
    /// by sending them to the allocated account channel
    fn handle_account_request<T: Encode + AccountRequest + SignedRequest>(
        &mut self, 
        request: T, 
        request_type: RequestType
    ) -> Result<u128, String> {
        if self.account_req_id == (self.state.get_account_request_results().len() as u128) - 1 {
            self.account_req_id = 0;
        }
        self.account_req_id += 1;
        let channel_id = self.get_account_channel_id(request.get_id())?;
        let sender = self.get_account_channel(channel_id)?;
        let payload = self.build_payload(&request)?;
        let result = sender.send(WrappedRequest { 
            id: self.account_req_id,
            request_type,
            payload,
            signature: request.get_signature().to_string(),
            nonce: request.get_nonce(),
            public_key: request.get_public_key().to_string()
        });
        match result {
            Ok(_) => Ok(self.account_req_id),
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
    pub fn list_market(&mut self, request: ListMarketRequest) -> Result<&Market, String> {
        let markets = self.state.get_markets();
        if markets.get(&request.symbol).is_none() {
            let market = Market::new(&request.symbol, request.max_price, request.min_price);
            let channel_id = markets.len() % self.market_channels.len();
            self.market_channel_by_symbol.insert(request.symbol.to_string(), channel_id);
            markets.insert(request.symbol.to_string(), market.clone());
            // we need exclusive write access here as this operation allocates heap memory
            let lock = self.markets_lock.write();
            // create new order book
            self.state.get_order_books().insert(request.symbol.to_string(), OrderBook::new(market));
            // drop the lock now that we're done
            drop(lock);
            Ok(markets.get(&request.symbol).unwrap())
        } else {
            Err("market already exists".to_string())
        }
    }

    /// List a new asset
    pub fn list_asset(&mut self, request: ListAssetRequest) -> Result<&Asset, String> {
        let assets = self.state.get_assets();
        let asset = Asset::new(request.blockchain, &request.symbol, request.decimals);
        if assets.get(&asset.get_id()).is_none() {
            assets.insert(asset.get_id(), asset.clone());
            // pre-allocate accounts for this asset
            let mut accounts = HashMap::new();
            for i in 0..10_000_000 {
                let id = i + 1;
                accounts.insert(id, Account::default());
            }
            let lock = self.accounts_lock.write();
            self.state.get_accounts().insert(asset.get_id(), accounts);
            drop(lock);
            Ok(assets.get(&asset.get_id()).unwrap())
        } else {
            Err("asset already exists".to_string())
        }
    }

    /// Submit a new order
    pub fn submit_order(&mut self, request: SubmitOrderRequest) -> Result<u128, String> {
        self.handle_market_request(request, RequestType::SubmitOrder)
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, request: CancelOrderRequest) -> Result<u128, String> {
        self.handle_market_request(request, RequestType::CancelOrder)
    }

    /// Credit account
    pub fn credit_account(&mut self, request: CreditAccountRequest) -> Result<u128, String> {
        self.handle_account_request(request, RequestType::CreditAccount)
    }

    /// Debit account
    pub fn debit_account(&mut self, request: CreditAccountRequest) -> Result<u128, String> {
        self.handle_account_request(request, RequestType::DebitAccount)
    }

    /// Lock account
    pub fn lock_account(&mut self, request: CreditAccountRequest) -> Result<u128, String> {
        self.handle_account_request(request, RequestType::LockAccount)
    }

    /// Unlock account
    pub fn unlock_account(&mut self, request: CreditAccountRequest) -> Result<u128, String> {
        self.handle_account_request(request, RequestType::UnlockAccount)
    }

    /// Get the request results in binary format for a list of known request IDs
    /// 
    /// Note: this function will block until all requests have either completed 
    /// successfully or otherwise failed
    pub fn get_results(&self, request_type: RequestType, request_ids: Vec<u128>) -> Vec<&Result<(RequestType, Vec<u8>), String>> {
        let mut results = Vec::new();
        for id in request_ids {
            loop {
                let request_results = match request_type {
                    RequestType::SubmitOrder => self.state.get_market_request_results(),
                    RequestType::CancelOrder => self.state.get_market_request_results(),
                    RequestType::CreditAccount => self.state.get_account_request_results(),
                    RequestType::DebitAccount => self.state.get_account_request_results(),
                    RequestType::LockAccount => self.state.get_account_request_results(),
                    RequestType::UnlockAccount => self.state.get_account_request_results(),
                };
                if let Some(Some(result)) = request_results.get(&id) {
                    results.push(result);
                    break;
                }
            }
        }
        results
    }

    /// Get the best bid and best ask for the specified market
    pub fn get_best_quote(&self, symbol: &String) -> Result<BestQuote, String> {
        self.state.get_best_quote(symbol)
    }

    /// Get all orders for the specified market
    pub fn get_orders_by_symbol(&self, symbol: &String) -> Result<Vec<Order>, String> {
        self.state.get_orders_by_symbol(symbol)
    }

    /// Get exchange stats
    pub fn get_stats() -> ExchangeStats {
        ExchangeStats {  }
    }
}
use crate::price_level::PriceLevel;
use crate::market::Market;
use crate::order::{
    OrderType,
    Side
};
use crate::request::{
    CancelOrderRequest, 
    SubmitOrderRequest
};

use std::collections::HashMap;

#[derive(Debug)]
pub struct BestQuote {
    pub best_bid_price: u64,
    pub best_ask_price: u64,
    pub best_bid_size: u64,
    pub best_ask_size: u64
}

pub struct OrderBook {
    market: Market,
    last_order_id: u64,
    best_quote: BestQuote,
    price_by_id: HashMap<u64, u64>,
    price_levels: Vec<PriceLevel>,
}

impl OrderBook {
    
    /// Create a new order book for given market
    pub fn new(
        market: Market
    ) -> Self {
        let mut price_levels = Vec::<PriceLevel>::new();
        let total_levels = market.get_max_price() - market.get_min_price();
        for i in 0..total_levels {
            price_levels.push(PriceLevel::new(market.get_min_price() + i));
        }
        let best_quote = BestQuote {
            best_ask_price: market.get_max_price(),
            best_bid_price: market.get_min_price(),
            best_bid_size: 0,
            best_ask_size: 0,
        };
        Self {
            last_order_id: 0,
            price_levels,
            best_quote,
            market,
            price_by_id: HashMap::new()
        }
    }

    /// Returns the best bid price
    pub fn get_best_bid_price(&self) -> u64 {
        self.best_quote.best_bid_price
    }

    /// Returns the best bid size
    pub fn get_best_bid_size(&self) -> u64 {
        self.best_quote.best_bid_size
    }

    /// Returns the best ask price
    pub fn get_best_ask_price(&self) -> u64 {
        self.best_quote.best_ask_price
    }

    /// Returns the best ask size
    pub fn get_best_ask_size(&self) -> u64 {
        self.best_quote.best_ask_size
    }
    
    /// Get the index that represents the lowest price level on the bid side of the book
    fn get_bid_from_index(&self) -> usize {
        0
    }
    
    /// Get the index that represents the highest price level on the bid side of the book
    fn get_bid_to_index(&self) -> usize {
        (self.best_quote.best_bid_price - self.market.get_min_price() + 1) as usize
    }
    
    /// Get the index that represents the lowest price level on the ask side of the book
    fn get_ask_from_index(&self) -> usize {
        (self.best_quote.best_ask_price - self.market.get_min_price()) as usize
    }
    
    /// Get the index that represents the highest price level on the ask side of the book
    fn get_ask_to_index(&self) -> usize {
        self.price_levels.len()
    }

    /// Get the bids
    pub fn get_bids(&self) -> Vec<&PriceLevel> {
        self.get_side_of_book(Side::Buy)
    }

    /// Get the asks
    pub fn get_asks(&self) -> Vec<&PriceLevel> {
        self.get_side_of_book(Side::Sell)
    }
    
    /// Get the price levels for specified side of the book
    pub fn get_side_of_book(&self, side: Side) -> Vec<&PriceLevel> {
        let mut side_of_book = Vec::new();
        let price_levels = self.get_price_levels(side);
        for price_level in price_levels.iter() {
            if price_level.get_size() > 0 {
                side_of_book.push(price_level);
            }
        }
        match side {
            Side::Buy => side_of_book.into_iter().rev().collect(),
            Side::Sell => side_of_book
        }
    }
    
    /// Get the price level index for a given price
    fn get_price_level_index(&self, price: u64) -> u64 {
        price - self.market.get_min_price()
    }

    /// Update the best bid price and size
    fn update_best_bid(&mut self) {
        let bids = self.get_price_levels(Side::Buy);
        let mut best_bid_price = 0;
        let mut best_bid_size = 0;
        for i in 0..bids.len() {
            let bid = bids.get(bids.len()-i-1).unwrap();
            let size = bid.get_size();
            if size > 0 {
                best_bid_price = bid.get_price();
                best_bid_size = size;
                break;
            }
        }
        if best_bid_price > 0 {
            self.best_quote.best_bid_price = best_bid_price;
            self.best_quote.best_bid_size = best_bid_size;
        } else {
            self.best_quote.best_bid_price = self.market.get_min_price();
            self.best_quote.best_bid_size = 0;
        }
    }

    /// Update the best ask price and size
    fn update_best_ask(&mut self) {
        let asks = self.get_price_levels(Side::Sell);
        let mut best_ask_price = 0;
        let mut best_ask_size = 0;
        for i in 0..asks.len() {
            let ask = asks.get(i).unwrap();
            let size = ask.get_size();
            if size > 0 {
                best_ask_price = ask.get_price();
                best_ask_size = size;
                break;
            }
        }
        if best_ask_price > 0 {
            self.best_quote.best_ask_price = best_ask_price;
            self.best_quote.best_ask_size = best_ask_size;
        } else {
            self.best_quote.best_ask_price = self.market.get_max_price();
            self.best_quote.best_ask_size = 0;
        }
    }

    /// Get immutable reference to the price levels representing specified side of the book
    fn get_price_levels(&self, side: Side) -> &[PriceLevel] {
        match side {
            Side::Buy => {
                let bid_from_index = self.get_bid_from_index();
                let bid_to_index = self.get_bid_to_index();
                &self.price_levels[bid_from_index..bid_to_index]
            },
            Side::Sell => {
                let ask_from_index = self.get_ask_from_index();
                let ask_to_index = self.get_ask_to_index();
                &self.price_levels[ask_from_index..ask_to_index]
            }
        }
    }

    /// Get a mutable reference to the price levels representing specified side of the book
    fn get_price_levels_mut(&mut self, side: Side) -> &mut [PriceLevel] {
        match side {
            Side::Buy => {
                let bid_from_index = self.get_bid_from_index();
                let bid_to_index = self.get_bid_to_index();
                &mut self.price_levels[bid_from_index..bid_to_index]
            },
            Side::Sell => {
                let ask_from_index = self.get_ask_from_index();
                let ask_to_index = self.get_ask_to_index();
                &mut self.price_levels[ask_from_index..ask_to_index]
            }
        }
    }

    /// Do matching when order crosses with the other side of the book
    fn handle_crossing_order(&mut self, request: &mut SubmitOrderRequest) {
        // get the passive side of the book
        let other_side = match request.side {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy
        };
        // get a mutable reference to the passive price levels
        let price_levels = self.get_price_levels_mut(other_side);
        // these variables are used to update the cursor for each side of the book
        let mut best_bid_price = 0;
        let mut best_ask_price = 0;
        // loop over the price levels on the book
        for i in 0..price_levels.len() {
            // iterate from the back when sell
            let offset = match request.side {
                Side::Buy => i,
                Side::Sell => price_levels.len() - i - 1
            };
            let price_level = &mut price_levels[offset];
            // skip levels with no size
            if price_level.get_size() == 0 {
                continue;
            }
            // end matching when aggressive order is fully filled
            if request.size == 0 {
                break;
            }
            // if order type == limit, then end matching when price is exceeded
            if request.order_type == OrderType::Limit && (
                (price_level.get_price() > request.price && request.side == Side::Buy) || 
                (price_level.get_price() < request.price && request.side == Side::Sell)
            ) {
                break;
            }
            let mut remove_ids = Vec::new();
            // loop over the orders at the price level
            let orders = price_level.get_orders_mut();
            for i in 0..orders.len() {
                let passive_order = orders.get_mut(i).unwrap();
                // if there's no order at this memory location, we can move to next price level
                if passive_order.get_price() == 0 {
                    break;
                }
                if passive_order.get_remaining() >= request.size {
                    // aggressive order is fully matched by this passive order
                    passive_order.set_remaining(passive_order.get_remaining() - request.size);
                    request.size = 0;
                    // update the best bid/ask price as we traverse the book
                    match request.side {
                        Side::Buy => best_ask_price = passive_order.get_price(),
                        Side::Sell => best_bid_price = passive_order.get_price()
                    }
                } else {
                    // passive order is fully matched by the aggressive order
                    request.size -= passive_order.get_remaining();
                    passive_order.set_remaining(0);
                    // update the best bid/ask price as we traverse the book
                    match request.side {
                        Side::Buy => best_ask_price = passive_order.get_price(),
                        Side::Sell => best_bid_price = passive_order.get_price()
                    }
                }
                // if the passive order is fully matched, then remove it from the price level
                if passive_order.get_remaining() == 0 {
                    remove_ids.push(passive_order.get_id());
                }
                // finish matching if the request is fully filled
                if request.size == 0 {
                    break;
                }
            }
            // remove matched orders from price level
            for id in remove_ids {
                // TODO: pass the list into this function then we only have to loop over the orders once
                price_level.remove_order(id);
            }
        }
        // if there's leftover size, add it to the book and update best bid/ask
        if request.size > 0 {
            match request.side {
                Side::Buy => {
                    best_bid_price = request.price;
                    best_ask_price = request.price + 1;
                },
                Side::Sell => {
                    best_ask_price = request.price;
                    best_bid_price = request.price - 1;
                }
            }
            self.handle_passive_order(request);
        }
        if best_bid_price > 0 {
            self.best_quote.best_bid_price = best_bid_price;
        }
        if best_ask_price > 0 {
            self.best_quote.best_ask_price = best_ask_price;
        }
        // this moves the best bid/ask to the first price level with size > 0
        self.update_best_bid();
        self.update_best_ask();
    }
    
    /// Add a passive order to the order book
    fn handle_passive_order(&mut self, request: &mut SubmitOrderRequest) {
        let index = self.get_price_level_index(request.price);
        let price_level = self.price_levels.get_mut(index as usize).unwrap();
        let order_price = request.price;
        let order_side = request.side;
        price_level.add_order(request.price, request.size, request.side, request.order_type, self.last_order_id);
        self.price_by_id.insert(self.last_order_id, request.price);
        let price_level_size = price_level.get_size();
        match order_side {
            Side::Buy => {
                if self.best_quote.best_bid_size == 0 {
                    self.best_quote.best_bid_price = price_level.get_price();
                    self.best_quote.best_bid_size = price_level_size;
                }
                if order_price > self.best_quote.best_bid_price {
                    self.best_quote.best_bid_price = price_level.get_price();
                    self.best_quote.best_bid_size = price_level_size;
                }
            },
            Side::Sell => {
                if self.best_quote.best_ask_size == 0 {
                    self.best_quote.best_ask_price = price_level.get_price();
                    self.best_quote.best_ask_size = price_level_size;
                }
                if order_price < self.best_quote.best_ask_price {
                    self.best_quote.best_ask_price = price_level.get_price();
                    self.best_quote.best_ask_size = price_level_size;                    
                }
            }
        }
    }
    
    /// Cancel order by specified id
    pub fn cancel_order(&mut self, request: &CancelOrderRequest) -> Result<bool, String> {
        let best_bid_price = self.get_best_bid_price();
        let best_ask_price = self.get_best_ask_price();
        match self.price_by_id.get(&request.id) {
            Some(price) => {
                let index = self.get_price_level_index(*price);
                let price_level = self.price_levels.get_mut(index as usize).unwrap();
                price_level.remove_order(request.id);
                // update the best bid/ask if the cancelled order is at the top of the book
                if *price == best_bid_price && price_level.get_size() == 0 {
                    self.update_best_bid();
                } else if *price == best_ask_price && price_level.get_size() == 0 {
                    self.update_best_ask();
                }
                Ok(true)
            }
            None => {
                Err("order not found".to_string())
            }
        }
    }

    /// Handle a new order submission request
    pub fn submit_order(&mut self, request: &mut SubmitOrderRequest) -> Result<u64, String> {
        if request.order_type == OrderType::Limit && request.price < self.market.get_min_price() {
            Err("order price is below min for market".to_string())
        } else if request.order_type == OrderType::Limit && request.price >= self.market.get_max_price() {
            Err("order price is above max for market".to_string())
        } else {
            self.last_order_id += 1;
            match request.order_type {
                OrderType::Limit => {
                    if (request.price >= self.best_quote.best_ask_price && request.side == Side::Buy) || 
                        (request.price <= self.best_quote.best_bid_price && request.side == Side::Sell) {
                        self.handle_crossing_order(request);
                    } else {
                        self.handle_passive_order(request);
                    }
                },
                OrderType::Market => {
                    request.price = 0;
                    self.handle_crossing_order(request);
                }
            }
            Ok(self.last_order_id)
        }
    }
}
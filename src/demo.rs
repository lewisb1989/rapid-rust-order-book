use std::time::SystemTime;

use crate::{
    exchange::Exchange, 
    order::{
        OrderType, 
        Side
    }, 
    request::{
        CancelOrderRequest, 
        ListMarketRequest, 
        RequestType, 
        SubmitOrderRequest
    }
};

fn list_markets(exchange: &mut Exchange) {
    let res = exchange.list_market(ListMarketRequest {
        symbol: "BTCUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        settlement_asset: "USDT".to_string(),
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string()
    });
    println!("listed market: {:?}", res);
    let res = exchange.list_market(ListMarketRequest {
        symbol: "ETHUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        settlement_asset: "USDT".to_string(),
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string()
    });
    println!("listed market: {:?}", res);
    let res = exchange.list_market(ListMarketRequest {
        symbol: "SOLUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        settlement_asset: "USDT".to_string(),
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string()
    });
    println!("listed market: {:?}", res);
    let res = exchange.list_market(ListMarketRequest {
        symbol: "LINKUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        settlement_asset: "USDT".to_string(),
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string()
    });
    println!("listed market: {:?}", res);
    let res = exchange.list_market(ListMarketRequest {
        symbol: "AAVEUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string(),
        settlement_asset: "USDT".to_string()
    });
    println!("listed market: {:?}", res);
    let res = exchange.list_market(ListMarketRequest {
        symbol: "OPUSDT".to_string(),
        max_price: 10_001,
        min_price: 1,
        settlement_asset: "USDT".to_string(),
        signature: "".to_string(),
        nonce: 0,
        public_key: "".to_string()
    });
    println!("listed market: {:?}", res);
}

fn add_limit_orders(exchange: &mut Exchange) {
    println!("adding limit orders for each market...");
    let mut request_ids = Vec::new();
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let total_orders = 10_000;
    let markets = exchange.get_markets();
    for price in 1..(total_orders+1) {
        for market in &markets {
            let side = if price < total_orders / 2 { Side::Buy } else { Side::Sell };
            let res = exchange.submit_order(SubmitOrderRequest {
                symbol: market.get_symbol().clone(), 
                price, 
                size: 1,
                side,
                order_type: OrderType::Limit,
                signature: String::from(""),
                nonce: 0,
                public_key: String::from("")
            });
            request_ids.push(res.unwrap());
        }
    }
    let results = exchange.get_results(RequestType::SubmitOrder, request_ids);
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    println!("total results = {}", results.len());
    let duration = end - start;
    let latency = duration / (total_orders as u128) / (markets.len() as u128);
    println!("insertion latency = {} ns", latency);
}

fn cancel_limit_orders(exchange: &mut Exchange) {
    println!("cancelling orders on each market...");
    let mut request_ids = Vec::new();
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let markets = exchange.get_markets();
    let mut total_cancellations = 0;
    for market in &markets {
        let orders = exchange.get_orders_by_symbol(market.get_symbol()).expect("cannot get orders").clone();
        for order in orders {
            let res = exchange.cancel_order(CancelOrderRequest { 
                symbol: market.get_symbol().clone(), 
                id: order.get_id(),
                signature: String::from(""),
                nonce: 0,
                public_key: String::from("")
            });
            request_ids.push(res.unwrap());
            total_cancellations += 1;
        }
    }
    let results = exchange.get_results(RequestType::CancelOrder, request_ids);
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    println!("total results = {}", results.len());
    let duration = end - start;
    let latency = duration / (total_cancellations as u128) / (markets.len() as u128);
    println!("cancellation latency = {} ns", latency);
}

fn do_market_orders(exchange: &mut Exchange) {
    println!("sending market orders to each book...");
    let mut request_ids = Vec::new();
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let markets = exchange.get_markets();
    let total_orders = 1_000;
    for market in &markets {
        for i in 0..total_orders {
            let side = if i < total_orders / 2 { Side::Buy } else { Side::Sell };
            let res = exchange.submit_order(SubmitOrderRequest {
                symbol: market.get_symbol().clone(), 
                price: 0, 
                size: 2,
                side,
                order_type: OrderType::Market,
                signature: String::from(""),
                nonce: 0,
                public_key: String::from("")
            });
            request_ids.push(res.unwrap());
        }
    }
    let results = exchange.get_results(RequestType::SubmitOrder, request_ids);
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    println!("total results = {}", results.len());
    let duration = end - start;
    let latency = duration / (total_orders as u128) / (markets.len() as u128);
    println!("matching latency = {} ns", latency);
}

fn display_top_of_book(exchange: &mut Exchange) {
    let markets = exchange.get_markets();
    for market in &markets {
        let best_quote = exchange.get_best_quote(market.get_symbol()).expect("cannot get best quote");
        println!("{} -> {:?}", market.get_symbol(), best_quote);
    }
}

pub fn run() {
    let mut exchange = Exchange::new();
    list_markets(&mut exchange);
    add_limit_orders(&mut exchange);
    display_top_of_book(&mut exchange);
    cancel_limit_orders(&mut exchange);
    display_top_of_book(&mut exchange);
    add_limit_orders(&mut exchange);
    display_top_of_book(&mut exchange);
    do_market_orders(&mut exchange);
    display_top_of_book(&mut exchange);
}
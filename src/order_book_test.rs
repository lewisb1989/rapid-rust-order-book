#[cfg(test)]
mod tests {

    use crate::order_book::OrderBook;
    use crate::market::Market;
    use crate::order::{
        OrderType, 
        Side
    };
    use crate::request::{CancelOrderRequest, SubmitOrderRequest};

    #[test]
    fn test_passive_non_crossing_orders() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 101, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 104, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 102, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 103, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 101);
        assert_eq!(order_book.get_best_bid_size(), 1);
        assert_eq!(order_book.get_best_ask_price(), 102);
        assert_eq!(order_book.get_best_ask_size(), 1);
        assert_eq!(bids.len(), 3);
        assert_eq!(asks.len(), 3);
        assert_eq!(bids.get(0).unwrap().get_price(), 101);
        assert_eq!(bids.get(0).unwrap().get_size(), 1);
        assert_eq!(bids.get(1).unwrap().get_price(), 100);
        assert_eq!(bids.get(1).unwrap().get_size(), 1);
        assert_eq!(bids.get(2).unwrap().get_price(), 99);
        assert_eq!(bids.get(2).unwrap().get_size(), 1);
        assert_eq!(asks.get(0).unwrap().get_price(), 102);
        assert_eq!(asks.get(0).unwrap().get_size(), 1);
        assert_eq!(asks.get(1).unwrap().get_price(), 103);
        assert_eq!(asks.get(1).unwrap().get_size(), 1);
        assert_eq!(asks.get(2).unwrap().get_price(), 104);
        assert_eq!(asks.get(2).unwrap().get_size(), 1);
    }

    #[test]
    fn test_crossing_bid_fully_matched() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let min_price = market.get_min_price();
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 2, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), min_price);
        assert_eq!(order_book.get_best_bid_size(), 0);
        assert_eq!(order_book.get_best_ask_price(), 99);
        assert_eq!(order_book.get_best_ask_size(), 1);
        assert_eq!(bids.len(), 0);
        assert_eq!(asks.len(), 1);
        assert_eq!(asks.get(0).unwrap().get_price(), 99);
        assert_eq!(asks.get(0).unwrap().get_size(), 1);
    }

    #[test]
    fn test_crossing_ask_fully_matched() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let max_price = market.get_max_price();
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 2, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 99);
        assert_eq!(order_book.get_best_bid_size(), 1);
        assert_eq!(order_book.get_best_ask_price(), max_price);
        assert_eq!(order_book.get_best_ask_size(), 0);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 0);
        assert_eq!(bids.get(0).unwrap().get_price(), 99);
        assert_eq!(bids.get(0).unwrap().get_size(), 1);
    }

    #[test]
    fn test_crossing_bid_partial_match() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 2, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 2, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 105, size: 10, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 100);
        assert_eq!(order_book.get_best_bid_size(), 6);
        assert_eq!(order_book.get_best_ask_price(), 105);
        assert_eq!(order_book.get_best_ask_size(), 10);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 1);
        assert_eq!(bids.get(0).unwrap().get_price(), 100);
        assert_eq!(bids.get(0).unwrap().get_size(), 6);
        assert_eq!(asks.get(0).unwrap().get_price(), 105);
        assert_eq!(asks.get(0).unwrap().get_size(), 10);
    }

    #[test]
    fn test_crossing_ask_partial_match() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 105, size: 2, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 103, size: 2, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 95, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 103, size: 10, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 95);
        assert_eq!(order_book.get_best_bid_size(), 10);
        assert_eq!(order_book.get_best_ask_price(), 103);
        assert_eq!(order_book.get_best_ask_size(), 6);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 1);
        assert_eq!(bids.get(0).unwrap().get_price(), 95);
        assert_eq!(bids.get(0).unwrap().get_size(), 10);
        assert_eq!(asks.get(0).unwrap().get_price(), 103);
        assert_eq!(asks.get(0).unwrap().get_size(), 6);
    }

    #[test]
    fn test_crossing_bid_fully_matched_on_book() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let min_price = market.get_min_price();
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 102, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), min_price);
        assert_eq!(order_book.get_best_bid_size(), 0);
        assert_eq!(order_book.get_best_ask_price(), 102);
        assert_eq!(order_book.get_best_ask_size(), 1);
        assert_eq!(bids.len(), 0);
        assert_eq!(asks.len(), 1);
        assert_eq!(asks.get(0).unwrap().get_price(), 102);
        assert_eq!(asks.get(0).unwrap().get_size(), 1);
    }

    #[test]
    fn test_crossing_ask_fully_matched_on_book() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let max_price = market.get_max_price();
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 95, size: 1, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 1, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 95);
        assert_eq!(order_book.get_best_bid_size(), 1);
        assert_eq!(order_book.get_best_ask_price(), max_price);
        assert_eq!(order_book.get_best_ask_size(), 0);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 0);
        assert_eq!(bids.get(0).unwrap().get_price(), 95);
        assert_eq!(bids.get(0).unwrap().get_size(), 1);
    }

    #[test]
    fn test_market_buy() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 10, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 1, order_type: OrderType::Market, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 99);
        assert_eq!(order_book.get_best_bid_size(), 10);
        assert_eq!(order_book.get_best_ask_price(), 100);
        assert_eq!(order_book.get_best_ask_size(), 9);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 1);
        assert_eq!(bids.get(0).unwrap().get_price(), 99);
        assert_eq!(bids.get(0).unwrap().get_size(), 10);
        assert_eq!(asks.get(0).unwrap().get_price(), 100);
        assert_eq!(asks.get(0).unwrap().get_size(), 9);
    }

    #[test]
    fn test_market_sell() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 99, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 10, order_type: OrderType::Limit, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 1, order_type: OrderType::Market, side: Side::Sell 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        let asks = order_book.get_side_of_book(Side::Sell);
        assert_eq!(order_book.get_best_bid_price(), 99);
        assert_eq!(order_book.get_best_bid_size(), 9);
        assert_eq!(order_book.get_best_ask_price(), 100);
        assert_eq!(order_book.get_best_ask_size(), 10);
        assert_eq!(bids.len(), 1);
        assert_eq!(asks.len(), 1);
        assert_eq!(bids.get(0).unwrap().get_price(), 99);
        assert_eq!(bids.get(0).unwrap().get_size(), 9);
        assert_eq!(asks.get(0).unwrap().get_price(), 100);
        assert_eq!(asks.get(0).unwrap().get_size(), 10);
    }

    #[test]
    fn test_submit_order_fails_with_price_below_min() {
        let market = Market::new("BTCUSD", 10_000, 50);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 10, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), "order price is below min for market");
    }


    #[test]
    fn test_submit_order_fails_with_price_above_max() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 1000000000, size: 10, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), "order price is above max for market");
    }

    #[test]
    fn test_cancel_order() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 100, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        assert_eq!(bids.len(), 1);
        let order_id = res.unwrap();
        let res = order_book.cancel_order(&mut CancelOrderRequest { symbol: market.get_symbol().to_string(), id: order_id });
        assert_eq!(res.is_ok(), true);
        let bids = order_book.get_side_of_book(Side::Buy);
        assert_eq!(bids.len(), 0);
    }

    #[test]
    fn test_cancel_order_fails_with_invalid_id() {
        let market = Market::new("BTCUSD", 10_000, 1);
        let mut order_book = OrderBook::new(market.clone());
        let res = order_book.submit_order(&mut SubmitOrderRequest { 
            symbol: market.get_symbol().to_string(), price: 100, size: 100, order_type: OrderType::Limit, side: Side::Buy 
        });
        assert_eq!(res.is_ok(), true);   
        let res = order_book.cancel_order(&mut CancelOrderRequest{symbol: market.get_symbol().to_string(), id: 100 });
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), "order not found");
    }

}
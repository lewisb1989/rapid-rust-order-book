use bincode::Decode;
use bincode::Encode;

use crate::order::Order;
use crate::order::Side;
use crate::order::OrderStatus;
use crate::order::OrderType;

const MAX_ORDERS_PER_LEVEL: usize = 200;

#[derive(Encode, Decode, Debug)]
pub struct PriceLevel {
    orders: Vec<Order>,
    order_cursor: usize,
    price: u64,
}

impl PriceLevel {

    /// Creates a price level with given price
    pub fn new(
        price: u64,
    ) -> Self {
        let mut orders: Vec<Order> = Vec::new();
        for _ in 0..MAX_ORDERS_PER_LEVEL {
            orders.push(Order::new(0, 0, Side::Buy, OrderStatus::Open, OrderType::Limit));
        }
        Self { 
            price, 
            order_cursor: 0,
            orders
        }
    }

    /// Returns mutable reference to active orders
    pub fn get_orders_mut(&mut self) -> &mut [Order] {
        &mut self.orders[0..self.order_cursor]
    }

    /// Returns immutable reference to active orders
    pub fn get_orders(&self) -> &[Order] {
        &self.orders[0..self.order_cursor]
    }
    
    /// Returns the total remaining volume available at this price level 
    pub fn get_size(&self) -> u64 {
        let mut total_size = 0;
        for order in &self.orders {
            if order.get_price() == 0 {
                break;
            }
            total_size += order.get_remaining();
        }
        total_size
    }
    
    /// Returns the price of this price level
    pub fn get_price(&self) -> u64 {
        self.price
    }
    
    /// Adds an order to this price level
    pub fn add_order(&mut self, price: u64, size: u64, side: Side, order_type: OrderType, id: u64) {
        if price != self.price {
            panic!("order price {} does not match level price {}", price, self.price);
        }
        if let Some(order) = self.orders.get_mut(self.order_cursor) {
            order.set_price(price);
            order.set_remaining(size);
            order.set_side(side);
            order.set_size(size);
            order.set_status(OrderStatus::Open);
            order.set_type(order_type);
            order.set_id(id);
            self.order_cursor += 1;
        } else {
            panic!("max orders at price level reached");
        }
    }
    
    /// Removes an order from this price level
    pub fn remove_order(&mut self, id: u64) {
        for (index, order) in self.get_orders_mut().iter_mut().enumerate() {
            if order.get_id() == id {
                order.set_price(0);
                order.set_remaining(0);
                order.set_side(Side::Buy);
                order.set_size(0);
                order.set_status(OrderStatus::Open);
                order.set_type(OrderType::Limit);
                order.set_id(0);
                self.orders[index..self.order_cursor].rotate_left(1);
                self.order_cursor -= 1;
                break;
            }
        }
    }
}
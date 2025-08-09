#[cfg(test)]
mod tests {

    use crate::order::{Order, Side, OrderType, OrderStatus};

    #[test]
    fn test_new() {
        let mut order = Order::new(100, 1, Side::Buy, OrderStatus::Open, OrderType::Limit);
        order.set_id(1);
        assert_eq!(order.get_id(), 1);
        assert_eq!(order.get_price(), 100);
        assert_eq!(order.get_remaining(), 1);
        assert_eq!(order.get_size(), 1);
    }

    #[test]
    fn test_encode_and_decode() {
        let order = Order::new(100, 1, Side::Buy, OrderStatus::Open, OrderType::Limit);
        let bytes = bincode::encode_to_vec(order, bincode::config::standard());
        assert_eq!(bytes.is_ok(), true);
        let (decoded_order, _): (Order, usize) = bincode::decode_from_slice(&bytes.unwrap(), bincode::config::standard()).unwrap();
        assert_eq!(decoded_order, order);
    }
}
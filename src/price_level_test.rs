#[cfg(test)]
mod tests {

    use crate::price_level::PriceLevel;
    use crate::order::{Side, OrderType};

    #[test]
    fn test_new() {
        let price_level = PriceLevel::new(100);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 0);
    }

    #[test]
    fn test_add_order() {
        let mut price_level = PriceLevel::new(100);
        price_level.add_order(100, 1, Side::Buy, OrderType::Limit, 1);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 1);
    }

    #[test]
    #[should_panic]
    fn test_add_order_fails_when_level_full() {
        let mut price_level = PriceLevel::new(100);
        for i in 0..201 {
            price_level.add_order(100, i+1, Side::Buy, OrderType::Limit, i+1);
            assert_eq!(price_level.get_price(), 100);
            assert_eq!(price_level.get_size(), i+1);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_order_fails_with_price_mismatch() {
        let mut price_level = PriceLevel::new(100);
        price_level.add_order(101, 1, Side::Buy, OrderType::Limit, 1);
    }

    #[test]
    fn test_remove_order() {
        let mut price_level = PriceLevel::new(100);
        for i in 0..3 {
            price_level.add_order(100, 1, Side::Buy, OrderType::Limit, i+1);
            assert_eq!(price_level.get_price(), 100);
            assert_eq!(price_level.get_size(), i+1);
        }
        assert_eq!(price_level.get_size(), 3);
        assert_eq!(price_level.get_orders().get(0).unwrap().get_price(), 100);
        assert_eq!(price_level.get_orders().get(1).unwrap().get_price(), 100);
        assert_eq!(price_level.get_orders().get(2).unwrap().get_price(), 100);
        price_level.remove_order(2);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 2);
        assert_eq!(price_level.get_orders().len(), 2);
        assert_eq!(price_level.get_orders().get(0).unwrap().get_price(), 100);
        assert_eq!(price_level.get_orders().get(1).unwrap().get_price(), 100);
        price_level.remove_order(1);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 1);
        assert_eq!(price_level.get_orders().len(), 1);
        assert_eq!(price_level.get_orders().get(0).unwrap().get_price(), 100);
        price_level.remove_order(3);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 0);
        price_level.remove_order(100);
        assert_eq!(price_level.get_price(), 100);
        assert_eq!(price_level.get_size(), 0);
        assert_eq!(price_level.get_orders().len(), 0);
    }

}
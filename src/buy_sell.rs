pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min_price = std::i32::MAX;
    prices.into_iter().for_each(|price| {
        profit = profit.max(price - min_price);
        min_price = min_price.min(price);
    });
    profit
}

#[cfg(test)]
mod buy_sell_test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_profit([7, 1, 5, 3, 6, 4].to_vec()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }
}

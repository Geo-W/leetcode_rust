pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut last_cost = i32::MAX;
    let mut profit = 0;

    for price in prices {
        if price > last_cost {
            profit += price - last_cost;
        }
        last_cost = price;
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }
}

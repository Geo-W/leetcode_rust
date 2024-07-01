use std::cmp::{max, min};

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }
    let mut dp = vec![(0, 0); prices.len() - 1];
    dp[0] = (min(prices[1], prices[0]), max(0, prices[1] - prices[0]));
    for idx in 1..dp.len() {
        let min_value = min(dp[idx - 1].0, prices[idx + 1]);
        dp[idx] = (min_value, max(dp[idx - 1].1, prices[idx + 1] - min_value));
    }
    dp.last().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}

use std::cmp::max;
use std::collections::HashMap;

pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let map = chars
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, c)| {
            acc.insert(c, vals[idx]);
            return acc;
        });
    let mut dp = s
        .chars()
        .map(|x| *map.get(&x).unwrap_or(&(x as i32 - 96)))
        .collect::<Vec<_>>();
    let mut ret = max(dp[0], 0);
    for idx in 1..dp.len() {
        dp[idx] = max(dp[idx - 1] + dp[idx], dp[idx]);
        ret = max(ret, dp[idx]);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000]),
            2
        );
    }
}

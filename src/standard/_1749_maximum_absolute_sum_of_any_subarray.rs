use std::cmp::{max, min};

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut ret = nums[0].abs();

    let mut last_dp = (nums[0], nums[0]);

    for idx in 1..nums.len() {
        last_dp.0 = max(last_dp.0 + nums[idx], nums[idx]);
        last_dp.1 = min(last_dp.1 + nums[idx], nums[idx]);
        ret = max(last_dp.0.abs(), ret);
        ret = max(last_dp.1.abs(), ret);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    }

    #[test]
    fn b() {
        assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    }
}


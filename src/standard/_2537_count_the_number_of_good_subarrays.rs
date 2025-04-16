// Double pointer + memorization

use std::collections::HashMap;

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    if nums.len() == 1 {
        return 0;
    }
    let mut ret = 0;

    let mut map = HashMap::new();

    let mut idx_l = 0;
    let mut idx_r = 0;

    map.insert(nums[idx_l], 1);
    let mut current_k = 0;
    let mut move_right_ptr = true;
    while idx_l < nums.len() || idx_r + 1 < nums.len() {
        match move_right_ptr && idx_r + 1 < nums.len() {
            true => {
                idx_r += 1;
                let entry = map.entry(nums[idx_r]).or_insert(0);
                current_k -= combinations(*entry, 2);
                *entry += 1;
                current_k += combinations(*entry, 2);
                if current_k >= k {
                    ret += nums.len() - idx_r;
                    move_right_ptr = false;
                }
            }
            false => {
                let entry = map.entry(nums[idx_l]).or_insert(0);
                current_k -= combinations(*entry, 2);
                *entry -= 1;
                current_k += combinations(*entry, 2);
                idx_l += 1;
                if current_k >= k {
                    ret += nums.len() - idx_r;
                } else {
                    move_right_ptr = true;
                }
            }
        }
    }

    ret as i64
}

fn combinations(n: i32, k: i32) -> i32 {
    if k == 0 || k == n {
        return 1;
    }
    if k > n {
        return 0;
    }
    let k = std::cmp::min(k, n - k);

    let mut numerator = 1;
    for i in 0..k {
        numerator *= n - i;
    }

    let mut denominator = 1;
    for i in 1..=k {
        denominator *= i;
    }

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(count_good(vec![1, 1, 1, 1, 1], 10), 1);
    }

    #[test]
    fn b() {
        assert_eq!(count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    }

    #[test]
    fn c() {
        assert_eq!(
            count_good(
                vec![
                    2, 3, 2, 3, 1, 1, 3, 3, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1
                ],
                10
            ),
            315
        );
    }
}

pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
    let mut ret = nums[0];

    for idx in 1..nums.len() {
        nums[idx] = std::cmp::max(nums[idx - 1] + nums[idx], nums[idx]);
        ret = std::cmp::max(ret, nums[idx]);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}

// f(1) = -2
// f(2) = max(f(1) + 2, 2)  1
// f(3) = max(f(2) + 3, 3)

pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut pos = 0;
    let mut neg = 0;
    let mut dp_pos = vec![0; nums.len()];
    let mut dp_neg = vec![0; nums.len()];
    if nums[0] > 0 {
        dp_pos[0] = nums[0];
        pos = nums[0];
    } else {
        dp_neg[0] = nums[0];
        neg = nums[0];
    }
    for (idx, num) in nums.into_iter().enumerate().skip(1) {
        if dp_pos[idx - 1] * num >= num {
            dp_pos[idx] = dp_pos[idx - 1] * num;
        } else {
            dp_pos[idx] = num;
        }
        pos = std::cmp::max(dp_pos[idx], pos);

        if dp_neg[idx - 1] * num <= num {
            dp_neg[idx] = dp_neg[idx - 1] * num;
        } else {
            dp_neg[idx] = num;
        }

        let arr = [dp_neg[idx - 1] * num, dp_pos[idx - 1] * num, num];
        dp_neg[idx] = *arr.iter().min().unwrap();
        dp_pos[idx] = *arr.iter().max().unwrap();
        pos = std::cmp::max(dp_pos[idx], pos);
        neg = std::cmp::min(dp_neg[idx], neg);
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn b() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn c() {
        assert_eq!(max_product(vec![2, -1, 1, 1]), 2);
    }
}

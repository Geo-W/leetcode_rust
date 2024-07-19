pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut acc = 0;
    let mut last = i32::MAX;
    let mut nums_s = vec![];
    for num in nums {
        if num != last {
            if acc > 0 {
                nums_s.push((last, acc));
            }
            last = num;
            acc = 1;
        } else {
            acc += 1;
        }
    }
    nums_s.push((last, acc));
    let mut ret = 0;

    let mut dp = vec![];
    for (idx, &(num, cnt)) in nums_s.iter().enumerate() {
        let mut tmp = num * cnt;
        if idx > 0 {
            if num - 1 == nums_s[idx - 1].0 {
                if idx > 1 {
                    tmp += dp[idx - 2];
                }
            } else {
                tmp += dp[idx - 1];
            }
        }
        ret = std::cmp::max(ret, tmp);
        dp.push(ret);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn b() {
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}

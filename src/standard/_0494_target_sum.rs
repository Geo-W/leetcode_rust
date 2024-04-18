fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    let diff = sum - target;

    if diff < 0 || diff % 2 != 0 {
        return 0;
    }

    let neg = (diff / 2) as usize;
    let n = nums.len();
    let mut dp = vec![vec![0; neg + 1]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        let num = nums[i - 1] as usize;
        for j in 0..=neg {
            dp[i][j] = dp[i - 1][j];
            if j >= num {
                dp[i][j] += dp[i - 1][j - num];
            }
        }
    }

    dp[n][neg]
}

pub fn find_target_sum_ways_dfs(nums: Vec<i32>, target: i32) -> i32 {
    let mut ret = 0;

    fn dfs(nums: &Vec<i32>, target: i32, ret: &mut i32, cur_idx: usize, cur_sum: i32) {
        if cur_idx < nums.len() {
            for i in [-1, 1] {
                dfs(nums, target, ret, cur_idx + 1, cur_sum + i * nums[cur_idx])
            }
        } else {
            if cur_sum == target {
                *ret += 1;
            }
        }
    }

    dfs(&nums, target, &mut ret, 0, 0);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}



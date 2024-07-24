use std::collections::HashMap;

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize]; m as usize];
    let mut ret = 0;
    for col in 0..(n as usize) {
        dp[0][col] += 1;
        dp[m as usize - 1][col] += 1;
    }

    for row in 0..(m as usize) {
        dp[row][0] += 1;
        dp[row][n as usize - 1] += 1;
    }

    fn dfs(
        dp: &Vec<Vec<i32>>,
        cur_move: i32,
        max_move: i32,
        cache_map: &mut HashMap<(i32, usize, usize), i32>,
        row: usize,
        col: usize,
    ) -> i32 {
        let mut r = 0;
        if let Some(v) = cache_map.get(&(cur_move, row, col)) {
            return *v;
        }
        if cur_move < max_move {
            r += dp[row][col];
            if row > 0 {
                r += dfs(dp, cur_move + 1, max_move, cache_map, row - 1, col);
                r %= 1000000007;
            }
            if row < dp.len() - 1 {
                r += dfs(dp, cur_move + 1, max_move, cache_map, row + 1, col);
                r %= 1000000007;
            }
            if col > 0 {
                r += dfs(dp, cur_move + 1, max_move, cache_map, row, col - 1);
                r %= 1000000007;
            }
            if col < dp[0].len() - 1 {
                r += dfs(dp, cur_move + 1, max_move, cache_map, row, col + 1);
                r %= 1000000007;
            }
        }
        cache_map.insert((cur_move, row, col), r % 1000000007);
        return r % 1000000007;
    }

    ret += dfs(
        &dp,
        0,
        max_move,
        &mut HashMap::new(),
        start_row as usize,
        start_column as usize,
    ) % 1000000007;

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
    }

    #[test]
    fn b() {
        assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
    }

    #[test]
    fn c() {
        assert_eq!(find_paths(8, 7, 10, 1, 5), 99);
    }

    #[test]
    fn d() {
        assert_eq!(find_paths(8, 50, 23, 5, 26), 914783380);
    }
}

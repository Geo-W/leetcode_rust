pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![-1; grid[0].len()]; grid.len()];
    dp[0] = grid[0].clone();
    for row_idx in 1..grid.len() {
        for col_idx in 0..grid[0].len() {
            let m = grid[row_idx - 1]
                .iter()
                .enumerate()
                .map(|(last_row_col_idx, &x)| (last_row_col_idx, x, x as usize))
                .map(|(last_row_col_idx, x, xusize)| {
                    (last_row_col_idx, x, &move_cost[xusize][col_idx])
                })
                .map(|(last_row_col_idx, _, move_c)| {
                    dp[row_idx - 1][last_row_col_idx] + grid[row_idx][col_idx] + *move_c
                })
                .min()
                .unwrap();
            dp[row_idx][col_idx] = m;
        }
    }
    *dp.last().unwrap().iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            min_path_cost(
                vec_vec![[5, 3], [4, 0], [2, 1]],
                vec_vec![[9, 8], [1, 5], [10, 12], [18, 6], [2, 4], [14, 3]]
            ),
            17
        );
    }
}

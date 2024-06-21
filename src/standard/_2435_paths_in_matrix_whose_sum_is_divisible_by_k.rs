pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp = vec![vec![vec![0; k as usize]; grid[0].len()]; grid.len()];
    dp[0][0][(grid[0][0] % k) as usize] = 1;
    let new_map = vec![0; k as usize];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            unsafe {
                let mut s1: *const Vec<i32> = &new_map;
                let mut s2: *const Vec<i32> = &new_map;
                if x > 0 {
                    s1 = &dp[y][x - 1];
                }
                if y > 0 {
                    s2 = &dp[y - 1][x];
                }
                for key in 0..(k as usize) {
                    for raw_ptr in [s1, s2] {
                        if (*raw_ptr)[key] > 0 {
                            dp[y][x][(key + grid[y][x] as usize) % k as usize] = (dp[y][x]
                                [(key + grid[y][x] as usize) % k as usize]
                                + (*raw_ptr)[key])
                                % 1000000007;
                        }
                    }
                }
            }
        }
    }
    dp.last().unwrap().last().unwrap()[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            number_of_paths(vec_vec![[5, 2, 4], [3, 0, 5], [0, 7, 2]], 3),
            2
        );
    }
}

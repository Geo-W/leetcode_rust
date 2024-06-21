use std::cmp::max;

pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![(-1, -1); grid[0].len()]; grid.len()];
    for y in 0..dp.len() {
        for x in 0..dp[0].len() {
            let mut tmp = vec![];
            if x > 0 {
                let s1 = dp[y][x - 1];
                tmp.push(s1.0 * grid[y][x] as i64);
                tmp.push(s1.1 * grid[y][x] as i64);
            }
            if y > 0 {
                let s2 = dp[y - 1][x];
                tmp.push(s2.0 * grid[y][x] as i64);
                tmp.push(s2.1 * grid[y][x] as i64);
            }
            if tmp.is_empty() {
                dp[y][x] = (grid[y][x] as i64, grid[y][x] as i64);
            } else {
                dp[y][x] = (*tmp.iter().max().unwrap(), *tmp.iter().min().unwrap());
            }
        }
    }

    let s = dp.last().unwrap().last().unwrap();
    return if s.0 < 0 && s.1 < 0 {
        -1
    } else {
        (max(s.0, s.1) % 1000000007) as i32
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            max_product_path(vec_vec![[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]]),
            -1
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            max_product_path(vec_vec![[1, -2, 1], [1, -2, 1], [3, -4, 1]]),
            8
        );
    }
}

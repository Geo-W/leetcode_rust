pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
    for row_idx in 1..grid.len() {
        for col_idx in 0..grid[0].len() {
            let (min_idx, &min_val) = grid[row_idx - 1]
                .iter()
                .enumerate()
                .min_by_key(|&(_, &num)| num)
                .unwrap();
            let (_, &sub_min_val) = grid[row_idx - 1]
                .iter()
                .enumerate()
                .filter(|x| x.0 != min_idx)
                .min_by_key(|&(_, &num)| num)
                .unwrap();

            let m = if col_idx == min_idx {
                sub_min_val
            } else {
                min_val
            };
            grid[row_idx][col_idx] = m + grid[row_idx][col_idx];
        }
    }
    *grid.last().unwrap().iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            min_falling_path_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            13
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            min_falling_path_sum(vec_vec![[1, 2, 3], [6, 5, 4], [7, 8, 9]]),
            12
        );
    }

    #[test]
    fn c() {
        assert_eq!(min_falling_path_sum(vec_vec![[7]]), 7);
    }
}

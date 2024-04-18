pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut matrix = vec![vec![0; grid[0].len()]; grid.len()];

    let mut tmp = 0;
    for x in 0..grid[0].len() {
        tmp += grid[0][x];
        matrix[0][x] = tmp;
    }
    tmp = 0;
    for y in 0..grid.len() {
        tmp += grid[y][0];
        matrix[y][0] = tmp;
    }

    for y in 1..grid.len() {
        for x in 1..grid[0].len() {
            matrix[y][x] = std::cmp::min(matrix[y - 1][x], matrix[y][x - 1]) + grid[y][x];
        }
    }

    *matrix.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(min_path_sum(vec_vec![[1, 3, 1], [1, 5, 1], [4, 2, 1]]), 7);
    }

    #[test]
    fn b() {
        assert_eq!(min_path_sum(vec_vec![[1, 2, 3], [4, 5, 6]]), 12);
    }

    #[test]
    fn c() {
        assert_eq!(min_path_sum(vec_vec![[1, 2], [1, 1]]), 3);
    }
}

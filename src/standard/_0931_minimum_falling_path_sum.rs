pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    for y in 1..matrix.len() {
        for x in 0..matrix.len() {
            let start = if x > 0 { x - 1 } else { 0 };
            let end = if x == matrix.len() - 1 { x } else { x + 1 };
            matrix[y][x] = (start..=end)
                .map(|x_lastline| matrix[y - 1][x_lastline] + matrix[y][x])
                .min()
                .unwrap();
        }
    }
    *matrix[matrix.len() - 1].iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            min_falling_path_sum(vec_vec![[2, 1, 3], [6, 5, 4], [7, 8, 9]]),
            13
        );
    }

    #[test]
    fn b() {
        assert_eq!(min_falling_path_sum(vec_vec![[-19, 57], [-40, -5]]), -59);
    }
}

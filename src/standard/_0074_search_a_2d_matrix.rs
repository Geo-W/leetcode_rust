pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let x_len = matrix[0].len();
    let len = matrix.len() * matrix[0].len();
    let mut left: i32 = 0;
    let mut right: i32 = (len - 1) as i32;
    while left <= right {
        let current = ((right - left) / 2 + left) as usize;
        let y = current / x_len;
        let x = current - x_len * y;
        if matrix[y][x] > target {
            right = current as i32 - 1;
        } else if matrix[y][x] < target {
            left = current as i32 + 1;
        } else if matrix[y][x] == target {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            search_matrix(
                vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
                3
            ),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            search_matrix(
                vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}

pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for col_idx in 0..matrix[0].len() {
        let mut minus_idx = vec![];
        let mut max = -2;
        for row_idx in 0..matrix.len() {
            max = std::cmp::max(max, matrix[row_idx][col_idx]);
            if matrix[row_idx][col_idx] == -1 {
                minus_idx.push(row_idx);
            }
        }
        for row_idx in minus_idx {
            matrix[row_idx][col_idx] = max;
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            modified_matrix(vec_vec![[1, 2, -1], [4, -1, 6], [7, 8, 9]]),
            vec_vec![[1, 2, 9], [4, 8, 6], [7, 8, 9]]
        );
    }
}

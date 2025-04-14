pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix
        .into_iter()
        .any(|row| row.binary_search(&target).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            search_matrix(
                crate::vec_vec![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30]
                ],
                5,
            ),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            search_matrix(
                crate::vec_vec![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30]
                ],
                8,
            ),
            true
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            search_matrix(
                crate::vec_vec![
                    [1, 2, 3, 4, 5],
                    [6, 7, 8, 9, 10],
                    [11, 12, 13, 14, 15],
                    [16, 17, 18, 19, 20],
                    [21, 22, 23, 24, 25]
                ],
                15
            ),
            true
        );
    }
}

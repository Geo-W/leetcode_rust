pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for x in 0..len {
        for y in 0..len {
            if (x + y) < len {
                (matrix[y][x], matrix[len - 1 - x][len - 1 - y]) = (matrix[len - 1 - x][len - 1 - y], matrix[y][x]);
            }
        }
    }

    for y in 0..(len / 2) {
        matrix.swap(y, len - 1 - y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut m);
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn b() {
        let mut m = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8.10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut m);
        assert_eq!(m, vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ]);
    }
}

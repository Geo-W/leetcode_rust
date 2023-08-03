/// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
/// You must do it in place.
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut records = vec![];
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == 0 {
                records.push([x, y])
            }
        }
    }
    for record in records {
        matrix[record[1]] = vec![0; matrix[record[1]].len()];
        for i in 0..matrix.len() {
            matrix[i][record[0]] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![
            vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1],
        ];
        set_zeroes(&mut vec);
        assert_eq!(vec, vec![
            vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1],
        ]);
    }

    #[test]
    fn a() {
        let mut vec = vec![
            vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5],
        ];
        set_zeroes(&mut vec);
        assert_eq!(vec, vec![
            vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0],
        ]);
    }
}

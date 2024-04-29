pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![];

    let target = mat.len() + mat[0].len() - 2;

    for i in 0..=target {
        let x_start = if i < mat[0].len() {
            i
        } else {
            mat[0].len() - 1
        };
        let x_end = if target - i < mat[0].len() {
            mat[0].len() - 1 - (target - i)
        } else {
            0
        };

        if i % 2 == 0 {
            for x in x_end..=x_start {
                let y = i - x;
                ret.push(mat[y][x]);
            }
        } else {
            for x in (x_end..=x_start).rev() {
                let y = i - x;
                ret.push(mat[y][x])
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_diagonal_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            find_diagonal_order(vec_vec![
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9],
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9],
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9]
            ]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 1, 4, 2, 9, 3, 5, 7, 1, 8, 6, 9, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut matrix = vec![vec![0; n]; m];

    for i in 0..n {
        matrix[0][i] = 1;
    }
    for i in 0..m {
        matrix[i][0] = 1;
    }

    for x in 1..n {
        for y in 1..m {
            matrix[y][x] = matrix[y - 1][x] + matrix[y][x - 1];
        }
    }

    matrix[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(unique_paths(3, 7), 28);
    }
}

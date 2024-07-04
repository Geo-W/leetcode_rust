pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut tmp = vec![vec!['.'; n]; n];
    let mut col = vec![false; n];
    let mut dial_right = vec![false; 2 * n - 1];
    let mut dial_left = vec![false; 2 * n - 1];
    fn dfs(
        ret: &mut i32,
        tmp: &mut Vec<Vec<char>>,
        col: &mut Vec<bool>,
        dial_right: &mut Vec<bool>,
        dial_left: &mut Vec<bool>,
        n: usize,
        current_n: usize,
        y: usize,
    ) {
        if n == current_n {
            *ret += 1;
        }
        if y >= n {
            return;
        }
        for x in 0..tmp[0].len() {
            if tmp[y][x] != 'Q' && !col[x] && !dial_right[y + n - x - 1] && !dial_left[x + y] {
                tmp[y][x] = 'Q';
                col[x] = true;
                dial_right[y + n - x - 1] = true;
                dial_left[x + y] = true;
                dfs(
                    ret,
                    tmp,
                    col,
                    dial_right,
                    dial_left,
                    n,
                    1 + current_n,
                    y + 1,
                );
                col[x] = false;
                dial_right[y + n - x - 1] = false;
                dial_left[x + y] = false;
                tmp[y][x] = '.';
            }
        }
    }
    let mut ret = 0;
    dfs(
        &mut ret,
        &mut tmp,
        &mut col,
        &mut dial_right,
        &mut dial_left,
        n,
        0,
        0,
    );
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(total_n_queens(4), 2);
    }
}

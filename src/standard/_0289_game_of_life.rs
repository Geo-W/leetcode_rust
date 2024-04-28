use std::cmp::min;

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut change_list = vec![];
    for x in 0..board[0].len() {
        for y in 0..board.len() {
            let mut alive_cells = 0;
            for x_inner in x.saturating_sub(1)..=min(board[0].len() - 1, x + 1) {
                for y_inner in y.saturating_sub(1)..=min(board.len() - 1, y + 1) {
                    if board[y_inner][x_inner] == 1 && (x != x_inner || y != y_inner) {
                        alive_cells += 1;
                    }
                }
            }
            match board[y][x] {
                0 => {
                    if alive_cells == 3 {
                        change_list.push((x, y, 1));
                    }
                }
                1 => {
                    if alive_cells < 2 || alive_cells > 3 {
                        change_list.push((x, y, 0));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    for (x, y, des) in change_list {
        board[y][x] = des;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        let mut r = vec_vec![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
        game_of_life(&mut r);
        assert_eq!(r, vec_vec![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]);
    }
}

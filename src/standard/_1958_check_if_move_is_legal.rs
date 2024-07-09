pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    let changes = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];
    'outer: for (x, y) in changes {
        let mut have_middle_color = false;
        let mut r = r_move;
        let mut c = c_move;
        loop {
            r += y;
            c += x;
            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                if board[r as usize][c as usize] == '.' {
                    continue 'outer;
                }
                if board[r as usize][c as usize] != color {
                    have_middle_color = true;
                }
                if board[r as usize][c as usize] == color {
                    if have_middle_color {
                        return true;
                    } else {
                        continue 'outer;
                    }
                }
            } else {
                break;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let board = vec![
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            ["W", "B", "B", ".", "W", "W", "W", "B"],
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
        ]
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(check_move(board, 4, 3, 'B'), true);
    }

    #[test]
    fn b() {
        let board = vec![
            ["W", "W", ".", "B", ".", "B", "B", "."],
            ["W", "B", ".", ".", "W", "B", ".", "."],
            ["B", "B", "B", "B", "W", "W", "B", "."],
            ["W", "B", ".", ".", "B", "B", "B", "."],
            ["W", "W", "B", ".", "W", ".", "B", "B"],
            ["B", ".", "B", "W", ".", "B", ".", "."],
            [".", "B", "B", "W", "B", "B", ".", "."],
            ["B", "B", "W", ".", ".", "B", ".", "."],
        ]
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(check_move(board, 7, 4, 'B'), false);
    }
}

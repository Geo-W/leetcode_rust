use std::collections::{HashMap, HashSet};

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut row_map = [[false; 9]; 9];
    let mut col_map = [[false; 9]; 9];
    let mut bulk_map = [[false; 9]; 9];

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] != '.' {
                row_map[y][board[y][x].to_digit(10).unwrap() as usize - 1] = true;
                col_map[x][board[y][x].to_digit(10).unwrap() as usize - 1] = true;
                bulk_map[(y / 3) * 3 + (x / 3)][board[y][x].to_digit(10).unwrap() as usize - 1] =
                    true
            }
        }
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        row_map: &mut [[bool; 9]; 9],
        col_map: &mut [[bool; 9]; 9],
        bulk_map: &mut [[bool; 9]; 9],
        x_start: usize,
        y_start: usize,
        deep: i32,
    ) -> bool {
        'outer: for y in 0..board.len() {
            for x in 0..board[0].len() {
                if y < y_start {
                    continue;
                }
                if y == y_start && x < x_start {
                    continue;
                }
                if board[y][x] == '.' {
                    let mut all_false = true;
                    for num in 0..=8 {
                        if !row_map[y][num as usize]
                            && !col_map[x][num as usize]
                            && !bulk_map[(y / 3) * 3 + (x / 3)][num as usize]
                        {
                            all_false = false;
                            board[y][x] = std::char::from_digit(num + 1, 10).unwrap();
                            row_map[y][num as usize] = true;
                            col_map[x][num as usize] = true;
                            bulk_map[(y / 3) * 3 + (x / 3)][num as usize] = true;

                            let (next_x, next_y) = if x < board[0].len() - 1 {
                                (x + 1, y)
                            } else {
                                (0, y + 1)
                            };
                            if dfs(board, row_map, col_map, bulk_map, next_x, next_y, deep + 1) {
                                break 'outer;
                            }
                            board[y][x] = '.';
                            row_map[y][num as usize] = false;
                            col_map[x][num as usize] = false;
                            bulk_map[(y / 3) * 3 + (x / 3)][num as usize] = false;
                            all_false = true;
                        }
                    }
                    if all_false {
                        return false;
                    }
                }
            }
        }
        true
    }
    dfs(board, &mut row_map, &mut col_map, &mut bulk_map, 0, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut board = vec![
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .into_iter()
        .map(|x| x.map(|f| f.parse::<char>().unwrap()))
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
        solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
                ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
                ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
                ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
                ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
                ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
                ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
                ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
                ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
            ]
            .into_iter()
            .map(|x| x.map(|f| f.parse::<char>().unwrap()))
            .map(|x| x.to_vec())
            .collect::<Vec<_>>()
        );
    }
}

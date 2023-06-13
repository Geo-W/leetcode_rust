///Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    let mut horizon = vec![HashSet::<char>::new(); 9];
    let mut vertical = vec![HashSet::<char>::new(); 9];
    let mut bulk = vec![HashSet::<char>::new(); 9];

    for x in 0..9{
        for y in 0..9{
            if board[y][x] != '.' {
                let bulk_location = match (x,y){
                    (0..=2, 0..=2) => 0,
                    (3..=5, 0..=2) => 1,
                    (6..=8, 0..=2) => 2,
                    (0..=2, 3..=5) => 3,
                    (3..=5, 3..=5) => 4,
                    (6..=8, 3..=5) => 5,
                    (0..=2, 6..=8) => 6,
                    (3..=5, 6..=8) => 7,
                    (6..=8, 6..=8) => 8,
                    _ => panic!("")
                };
                if !horizon[x].insert(board[y][x]) || !vertical[y].insert(board[y][x]) || !bulk[bulk_location].insert(board[y][x]) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn is_valid_sudoku_str(board: Vec<Vec<&str>>) -> bool {
    use std::collections::HashSet;
    let mut horizon = vec![HashSet::<&str>::new(); 9];
    let mut vertical = vec![HashSet::<&str>::new(); 9];
    let mut bulk = vec![HashSet::<&str>::new(); 9];

    for x in 0..9{
        for y in 0..9{
            if board[y][x] != "." {
                let bulk_location = match (x,y){
                    (0..=2, 0..=2) => 0,
                    (3..=5, 0..=2) => 1,
                    (6..=8, 0..=2) => 2,
                    (0..=2, 3..=5) => 3,
                    (3..=5, 3..=5) => 4,
                    (6..=8, 3..=5) => 5,
                    (0..=2, 6..=8) => 6,
                    (3..=5, 6..=8) => 7,
                    (6..=8, 6..=8) => 8,
                    _ => panic!("")
                };
                if !horizon[x].insert(board[y][x]) || !vertical[y].insert(board[y][x]) || !bulk[bulk_location].insert(board[y][x]) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nvec {
    [$($elements:expr), *] => {
        {
            let mut out = vec![];
            $(
              out.push($elements.to_vec());
            )*
            out
        }
    };
}


    #[test]
    fn it_works() {
        let board = nvec![["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]];
        let result = is_valid_sudoku_str(board);
        assert_eq!(result, true);
    }

    #[test]
    fn a(){
        let board = nvec![["8","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]];
        assert_eq!(is_valid_sudoku_str(board), false);
    }
}




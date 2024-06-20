pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut ret = 0 ;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'X' {
                let mut is = true;
                if y > 0 {
                    if board[y-1][x] == 'X' {
                        is = false;
                    }
                }
                if x > 0 {
                    if board[y][x-1] == 'X' {
                        is = false;
                    }
                }
                if is {
                    ret +=1;
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(count_battleships(vec![
            vec!['X','.','.','X'],
            vec!['.','.','.','X'],
            vec!['.','.','.','X'],
            vec!['.','.','.','.'],
        ]), 2);
    }
}

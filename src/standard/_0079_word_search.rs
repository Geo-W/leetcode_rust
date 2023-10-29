use std::collections::HashSet;

/// Given an m x n grid of characters board and a string word, search for the given word in sequence.
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    // Without using HashSet for storing past route, instead modify the past route in original matrix can get a much better performance.
    fn recursive_find(
        x: usize,
        y: usize,
        board: &Vec<Vec<char>>,
        words: &Vec<char>,
        word_idx: usize,
        past_route: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if board[y][x] == words[word_idx] {
            if word_idx + 1 == words.len() {
                return true;
            }
            past_route.insert((x, y));
            if y + 1 < board.len() {
                if !past_route.contains(&(x, y + 1)) {
                    if recursive_find(x, y + 1, board, words, word_idx + 1, past_route) == true {
                        return true;
                    }
                }
            }

            if y >= 1 {
                if !past_route.contains(&(x, y - 1)) {
                    if recursive_find(x, y - 1, board, words, word_idx + 1, past_route) == true {
                        return true;
                    }
                }
            }

            if x + 1 < board[0].len() {
                if !past_route.contains(&(x + 1, y)) {
                    if recursive_find(x + 1, y, board, words, word_idx + 1, past_route) == true {
                        return true;
                    }
                }
            }

            if x >= 1 {
                if !past_route.contains(&(x - 1, y)) {
                    if recursive_find(x - 1, y, board, words, word_idx + 1, past_route) == true {
                        return true;
                    }
                }
            }
            past_route.remove(&(x, y));
        }

        false
    }
    let words = word.chars().collect::<Vec<char>>();
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == words[0] {
                let mut past_route = HashSet::new();
                let b = recursive_find(x, y, &board, &words, 0, &mut past_route);
                if b == true {
                    return true;
                }
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
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_string(),
            ),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB".to_string(),
            ),
            false
        );
    }

    #[test]
    fn c() {
        assert_eq!(exist(vec![vec!['a']], "a".to_string()), true);
    }
}

pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    let mut dp = vec![vec![(0, 0); board[0].len()]; board.len()];
    for y in (0..board.len()).rev() {
        for x in (0..board[0].len()).rev() {
            let mut compare = vec![];
            if x < board[0].len() - 1 && dp[y][x + 1].1 != 0 {
                compare.push(dp[y][x + 1]);
            }
            if y < board.len() - 1 && dp[y + 1][x].1 != 0 {
                compare.push(dp[y + 1][x]);
            }
            if x < board[0].len() - 1
                && y < board.len() - 1
                && compare.is_empty()
                && dp[y + 1][x + 1].1 != 0
            {
                compare.push(dp[y + 1][x + 1]);
            }
            // dbg!(&compare);
            if &board[y][x..=x] == "X" {
                dp[y][x] = (0, 0);
            } else if &board[y][x..=x] == "S" {
                dp[y][x] = (0, 1);
            } else {
                let score = if &board[y][x..=x] == "E" {
                    0
                } else {
                    board[y][x..=x].parse::<i32>().unwrap()
                };
                match compare.len() {
                    0 => dp[y][x] = (0, 0),
                    1 => dp[y][x] = (compare[0].0 + score, compare[0].1),
                    2 => {
                        if compare[0].0 == compare[1].0 {
                            dp[y][x] = (
                                compare[0].0 + score,
                                (compare[0].1 + compare[1].1) % 1000000007,
                            );
                        } else if compare[0].0 > compare[1].0 {
                            dp[y][x] = (compare[0].0 + score, compare[0].1);
                        } else {
                            dp[y][x] = (compare[1].0 + score, compare[1].1);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    for d in &dp {
        println!("{:?}", d);
    }
    vec![dp[0][0].0, dp[0][0].1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            paths_with_max_score(vec_string!["E23", "2X2", "12S"]),
            vec![7, 1]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            paths_with_max_score(vec_string!["E11", "XXX", "11S"]),
            vec![0, 0]
        );
    }

    #[test]
    fn c() {
        assert_eq!(paths_with_max_score(vec_string!["EX", "XS"]), vec![0, 1]);
    }

    #[test]
    fn d() {
        assert_eq!(
            paths_with_max_score(vec_string![
                "E11345", "X452XX", "3X43X4", "44X312", "23452X", "1342XS"
            ]),
            vec![27, 1]
        );
    }
}

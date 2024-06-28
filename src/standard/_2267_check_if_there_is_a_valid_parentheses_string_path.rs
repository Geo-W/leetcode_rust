pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let mut dp = vec![vec![vec![false; grid[0].len() + grid.len()]; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 && y == 0 {
                if grid[y][x] == ')' {
                    return false;
                } else {
                    dp[y][x][1] = true;
                }
                continue;
            }

            let mut func = |x2: usize, y2: usize| {
                let last: *const Vec<bool> = &dp[y2][x2];
                unsafe {
                    for (idx, &i) in (&*last).iter().enumerate() {
                        if i != true || idx > (grid[0].len() + grid.len()) / 2 {
                            continue;
                        }
                        match grid[y][x] {
                            '(' => {
                                dp[y][x][idx + 1] = true;
                            }
                            ')' => {
                                if idx > 0 {
                                    dp[y][x][idx - 1] = true;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            };

            if x > 0 {
                func(x - 1, y);
            }

            if y > 0 {
                func(x, y - 1);
            }
        }
    }
    dp.last().unwrap().last().unwrap()[0]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            has_valid_path(vec![
                vec!['(', '(', '('],
                vec![')', '(', ')'],
                vec!['(', '(', ')'],
                vec!['(', '(', ')'],
            ]),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            has_valid_path(vec![vec!['(', ')'], vec!['(', ')'], vec!['(', '(']]),
            false
        );
    }

    #[test]
    fn c() {
        assert_eq!(has_valid_path(vec![vec!['(']]), false);
    }
}

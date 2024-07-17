pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    use std::collections::VecDeque;
    let mut ret = 0;
    let mut to_be_processed = VecDeque::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '1' {
                ret += 1;
                to_be_processed.push_back((y, x));
                while !to_be_processed.is_empty() {
                    for _ in 0..to_be_processed.len() {
                        let (y, x) = to_be_processed.pop_front().unwrap();
                        if grid[y][x] == '1' {
                            grid[y][x] = '0';
                            if y + 1 < grid.len() {
                                to_be_processed.push_back((y + 1, x));
                            }
                            if x + 1 < grid[0].len() {
                                to_be_processed.push_back((y, x + 1));
                            }
                            if x > 0 {
                                to_be_processed.push_back((y, x - 1));
                            }
                            if y > 0 {
                                to_be_processed.push_back((y - 1, x));
                            }
                        }
                    }
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
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
    }
}

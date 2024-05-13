use std::collections::VecDeque;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let mut queue = VecDeque::new();
    for y in grid.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            if *x.1 == 2 {
                queue.push_back((x.0, y.0));
            }
        }
    }

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();
            if x > 0 && grid[y][x - 1] == 1 {
                grid[y][x - 1] = 2;
                queue.push_back((x - 1, y));
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] == 1 {
                grid[y][x + 1] = 2;
                queue.push_back((x + 1, y));
            }
            if y > 0 && grid[y - 1][x] == 1 {
                grid[y - 1][x] = 2;
                queue.push_back((x, y - 1));
            }
            if y < grid.len() - 1 && grid[y + 1][x] == 1 {
                grid[y + 1][x] = 2;
                queue.push_back((x, y + 1));
            }
        }
        ret += 1;
    }

    match grid.iter().all(|x| x.iter().all(|x| *x != 1)) {
        false => -1,
        true => {
            if ret > 0 {
                ret - 1
            } else {
                ret
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            oranges_rotting(vec_vec![[2, 1, 1], [1, 1, 0], [0, 1, 1]]),
            4
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            oranges_rotting(vec_vec![
                [2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            ]),
            20
        );
    }

    #[test]
    fn c() {
        assert_eq!(oranges_rotting(vec_vec![[0]]), 0);
    }
}

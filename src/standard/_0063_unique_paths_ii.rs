pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if *obstacle_grid.last().unwrap().last().unwrap() == 1 {
        return 0;
    }
    for x in 0..obstacle_grid[0].len() {
        if obstacle_grid[0][x] == 1 {
            break;
        }
        obstacle_grid[0][x] = -1;
    }

    for y in 0..obstacle_grid.len() {
        if obstacle_grid[y][0] == 1 {
            break;
        }
        obstacle_grid[y][0] = -1;
    }

    for y in 1..obstacle_grid.len() {
        for x in 0..obstacle_grid[0].len() {
            if obstacle_grid[y][x] == 1 {
                continue;
            }
            let top = if obstacle_grid[y - 1][x] != 1 {
                obstacle_grid[y - 1][x]
            } else {
                0
            };
            let left = if x > 0 && obstacle_grid[y][x - 1] != 1 {
                obstacle_grid[y][x - 1]
            } else {
                0
            };
            obstacle_grid[y][x] = top + left;
        }
    }
    -*obstacle_grid.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            unique_paths_with_obstacles(vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
            2
        );
    }
}

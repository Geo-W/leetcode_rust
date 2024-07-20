pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut empty_nodes = vec![];
    let mut multiple_nodes = vec![];
    let mut ret = usize::MAX;

    for y in 0..3 {
        for x in 0..3 {
            if grid[y][x] == 0 {
                empty_nodes.push((x, y));
            }
            if grid[y][x] > 1 {
                multiple_nodes.push(((x, y), grid[y][x] - 1));
            }
        }
    }

    fn dfs(
        empty_nodes: &Vec<(usize, usize)>,
        idx: usize,
        multiple_nodes: &mut Vec<((usize, usize), i32)>,
        ret: &mut usize,
        cul: usize,
    ) {
        if idx == empty_nodes.len() {
            *ret = std::cmp::min(*ret, cul);
            return;
        }
        let empty_node = empty_nodes[idx];
        for idx_node in 0..multiple_nodes.len() {
            if multiple_nodes[idx_node].1 == 0 {
                continue;
            }
            let diff = multiple_nodes[idx_node].0 .0.abs_diff(empty_node.0)
                + multiple_nodes[idx_node].0 .1.abs_diff(empty_node.1);
            multiple_nodes[idx_node].1 -= 1;
            dfs(empty_nodes, idx + 1, multiple_nodes, ret, cul + diff);
            multiple_nodes[idx_node].1 += 1;
        }
    }

    dfs(&empty_nodes, 0, &mut multiple_nodes, &mut ret, 0);
    ret as i32
}

#[cfg(test)]
mod tests {
    use crate::vec_vec;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_moves(vec_vec![[1, 1, 0], [1, 1, 1], [1, 2, 1]]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(minimum_moves(vec_vec![[3, 2, 0], [0, 1, 0], [0, 3, 0]]), 7);
    }
}

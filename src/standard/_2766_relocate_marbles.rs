pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut set = nums.into_iter().collect::<std::collections::HashSet<_>>();
    for idx in 0..move_from.len() {
        if set.get(&move_from[idx]).is_some() {
            set.remove(&move_from[idx]);
            set.insert(move_to[idx]);
        }
    }

    let mut ret = set.into_iter().collect::<Vec<_>>();
    ret.sort_unstable();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5]),
            vec![5, 6, 8, 9]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            relocate_marbles(
                vec![3, 4],
                vec![4, 3, 1, 2, 2, 3, 2, 4, 1],
                vec![3, 1, 2, 2, 3, 2, 4, 1, 1],
            ),
            vec![1]
        );
    }
}

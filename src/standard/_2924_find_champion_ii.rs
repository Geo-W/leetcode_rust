use std::collections::HashSet;

pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut winner = (0..n).collect::<HashSet<_>>();
    for edge in edges.into_iter().rev() {
        winner.remove(&edge[1]);
    }

    if winner.len() == 1 {
        winner.into_iter().next().unwrap()
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(find_champion(3, vec_vec![[0, 1], [1, 2]]), 0);
    }

    #[test]
    fn b() {
        assert_eq!(find_champion(4, vec_vec![[0, 2], [1, 3], [1, 2]]), -1);
    }

    #[test]
    fn c() {
        assert_eq!(find_champion(1, vec![]), 0);
    }

    #[test]
    fn d() {
        assert_eq!(find_champion(3, vec_vec![[0, 1]]), -1);
    }
}

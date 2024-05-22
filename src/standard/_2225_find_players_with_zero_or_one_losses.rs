use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let map = matches.into_iter().fold(HashMap::new(), |mut acc, x| unsafe {
        acc.entry(*x.get_unchecked(0)).or_insert((0, 0)).0 += 1;
        acc.entry(*x.get_unchecked(1)).or_insert((0, 0)).1 += 1;
        return acc;
    });
    let mut ret = vec![vec![]; 2];
    for (idx, (win, loss)) in map {
        if loss == 0 {
            ret[0].push(idx);
        } else if loss == 1 {
            ret[1].push(idx);
        }
    }
    ret[0].sort_unstable();
    ret[1].sort_unstable();

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_winners(vec_vec![
                [1, 3],
                [2, 3],
                [3, 6],
                [5, 6],
                [5, 7],
                [4, 5],
                [4, 8],
                [4, 9],
                [10, 4],
                [10, 9]
            ]),
            vec_vec![[1, 2, 10], [4, 5, 7, 8]]
        );
    }
}

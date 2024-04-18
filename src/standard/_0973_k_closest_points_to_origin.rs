use std::cmp::Reverse;

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut m = points
        .iter()
        .enumerate()
        .map(|(idx, x)| (Reverse(x[0].pow(2) + x[1].pow(2)), idx))
        .collect::<std::collections::BinaryHeap<(Reverse<i32>, usize)>>();
    let mut ret = vec![];
    for _ in 0..k {
        ret.push(points[m.pop().unwrap().1].clone());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(k_closest(vec_vec![[1, 3], [-2, 2]], 1), vec_vec![[-2, 2]]);
    }
}

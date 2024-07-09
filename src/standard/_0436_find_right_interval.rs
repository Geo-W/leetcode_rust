pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut intervals2 = intervals
        .iter()
        .enumerate()
        .map(|(idx, x)| (x, idx))
        .collect::<Vec<_>>();
    intervals2.sort_by_key(|x| x.0[0]);
    let mut ret = Vec::with_capacity(intervals2.len());

    for interval in &intervals {
        let idx = intervals2
            .binary_search_by_key(&interval[1], |x| x.0[0])
            .unwrap_or_else(|v| v);
        if idx >= intervals2.len() {
            ret.push(-1);
        } else {
            ret.push(intervals2[idx].1 as i32);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(find_right_interval(vec_vec![[1, 2]]), [-1]);
    }

    #[test]
    fn b() {
        assert_eq!(
            find_right_interval(vec_vec![[3, 4], [2, 3], [1, 2]]),
            vec![-1, 0, 1]
        );
    }
}

use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums.into_iter()
        .fold(HashMap::new(), |mut cur, nxt| {
            *cur.entry(nxt).or_insert(0) += 1;
            cur
        })
        .into_iter()
        .map(|(num, frq)| (frq, num))
        .collect::<BinaryHeap<(usize, i32)>>()
        .into_sorted_vec()
        .into_iter()
        .rev()
        .take(k as usize)
        .map(|(frq, num)| num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn b() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}

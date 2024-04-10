use std::collections::BinaryHeap;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    words
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut x, nxt| {
            *x.entry(nxt).or_insert(0) += 1;
            return x;
        })
        .into_iter()
        .map(|x| (x.1, std::cmp::Reverse(x.0)))
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .into_iter()
        .rev()
        .take(k as usize)
        .map(|x| x.1 .0)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            top_k_frequent(
                vec_string!["i", "love", "leetcode", "i", "love", "coding"],
                2,
            ),
            vec_string!["i", "love"]
        );
    }
}

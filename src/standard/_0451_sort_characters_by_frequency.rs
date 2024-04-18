use std::collections::{BinaryHeap, HashMap};
use std::iter::repeat;

pub fn frequency_sort(s: String) -> String {
    s.chars()
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0usize) += 1;
            return acc;
        })
        .into_iter()
        .map(|x| (x.1, x.0))
        .collect::<BinaryHeap<(usize, char)>>()
        .into_sorted_vec()
        .into_iter()
        .rev()
        .map(|x| repeat(x.1).take(x.0).collect::<String>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(frequency_sort("tree".to_string()), "eetr".to_string());
    }
}

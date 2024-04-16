pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn f(s: String) -> i32 {
        let mut min = u8::MAX;
        let mut ret = 0;
        for i in s.bytes() {
            if i < min {
                min = i;
                ret = 1;
            } else if i == min {
                ret += 1;
            }
        }
        ret
    }
    let mut vec = vec![];

    let count_map =
        words
            .into_iter()
            .map(|x| f(x))
            .fold(std::collections::HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert_with(|| {
                    vec.push(x);
                    0
                }) += 1;
                acc
            });
    vec.sort();

    queries
        .into_iter()
        .map(|x| f(x))
        .map(|x| {
            let pos = match vec.binary_search(&x) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            vec[pos..vec.len()]
                .iter()
                .map(|x| count_map.get(x).unwrap())
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            num_smaller_by_frequency(
                vec_string!["bbb", "cc"],
                vec_string!["a", "aa", "aaa", "aaaa"],
            ),
            vec![1, 2]
        );
    }
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (idx, num) in nums.into_iter().enumerate() {
        let entry = map.entry(num).or_insert_with(Vec::new);
        match entry.binary_search(&idx) {
            Ok(v) => {
                entry.insert(v, idx);
            }
            Err(v) => {
                entry.insert(v, idx);
            }
        }
    }

    map.into_iter()
        .any(|x| x.1.windows(2).any(|v| v[1] - v[0] <= k as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    }

    #[test]
    fn b() {
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    }

    #[test]
    fn c() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}

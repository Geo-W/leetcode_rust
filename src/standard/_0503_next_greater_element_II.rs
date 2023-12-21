use std::collections::HashMap;

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![-1; nums.len() * 2];
    let mut mono = vec![];
    for (idx, item) in nums.iter().chain(nums.iter()).enumerate() {
        while let Some(v) = mono.last() {
            let v = *v;
            if *item > nums[v % nums.len()] {
                ret[v] = *item;
                mono.pop();
            } else {
                break;
            }
        }
        mono.push(idx);
    }
    ret.truncate(nums.len());
    ret
}

pub fn next_greater_elements2(nums: Vec<i32>) -> Vec<i32> {
    let mut mono = vec![];
    let mut map = HashMap::new();
    for i in nums.iter().chain(nums.iter()) {
        while let Some(v) = mono.last() {
            let v = *v;
            if *i > v {
                map.entry(v).or_insert(vec![]).push(*i);
                mono.pop();
            } else {
                break;
            }
        }
        mono.push(*i);
    }
    nums.into_iter()
        .map(|x| match map.get_mut(&x) {
            Some(v) if !v.is_empty() => v.remove(0),
            _ => -1,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    }

    #[test]
    fn b() {
        assert_eq!(
            next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            next_greater_elements(vec![100, 1, 11, 1, 120, 111, 123, 1, -1, -100]),
            vec![120, 11, 120, 120, 123, 123, -1, 100, 100, 100]
        );
    }
}

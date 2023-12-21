use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut mono = vec![];
    let mut map = HashMap::new();
    for i in nums2 {
        while let Some(v) = mono.last()  {
            let v = *v;
            if i > v {
                map.insert(v,i);
                mono.pop();
            } else {
                break;
            }
        }
        mono.push(i);
    }
    nums1.into_iter().map(|x| *map.get(&x).unwrap_or(&-1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}

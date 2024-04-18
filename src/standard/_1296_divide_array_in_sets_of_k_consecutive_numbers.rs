use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() % k as usize != 0 {
        return false;
    }

    let mut to_removed = HashMap::new();

    let mut map = HashMap::new();
    let mut heap = BinaryHeap::new();

    for i in nums {
        *map.entry(i).or_insert(0) += 1;
        heap.push(Reverse(i));
    }

    while let Some(v) = heap.pop() {
        let v = v.0;
        if let Some(to_removed_count) = to_removed.get_mut(&v) {
            if *to_removed_count > 1 {
                *to_removed_count -= 1;
            } else {
                to_removed.remove(&v);
            }
        } else {
            let value = map.get_mut(&v).unwrap();
            if *value > 1 {
                *value -= 1;
            } else {
                map.remove(&v);
            }
            for i in 0..k - 1 {
                // check whether array contains next value
                if let Some(value) = map.get_mut(&(i + v + 1)) {
                    if *value > 1 {
                        *value -= 1;
                    } else {
                        map.remove(&(i + v + 1));
                    }
                    // this value is used in this array, so remove it when got it in the heap
                    *to_removed.entry(i + v + 1).or_insert(0) += 1;
                } else {
                    return false;
                }
            }
        }
    }


    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(is_possible_divide([1, 2, 3, 3, 4, 4, 5, 6].to_vec(), 4), true);
    }
}
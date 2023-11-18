use std::collections::{BinaryHeap, HashMap};

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    nums.into_iter().for_each(|x| {
        map.entry(bit_sum(x))
            .or_insert_with(BinaryHeap::new)
            .push(x)
    });
    map.iter_mut()
        .map(|x| x.1)
        .filter(|x| x.len() >= 2)
        .map(|x| x.pop().unwrap() + x.pop().unwrap())
        .max()
        .unwrap_or(-1)
}

fn bit_sum(i: i32) -> i32 {
    let mut i = i;
    let mut ret = 0;
    while i > 0 {
        ret += i % 10;
        i = i / 10;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(bit_sum(53), 8);
        assert_eq!(bit_sum(108), 9);
    }

    #[test]
    fn b() {
        assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    }

    #[test]
    fn c() {
        assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}

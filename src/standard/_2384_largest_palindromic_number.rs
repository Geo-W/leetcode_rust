use std::collections::{BinaryHeap, HashMap};

pub fn largest_palindromic(num: String) -> String {
    let map = num.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        return acc;
    });

    let mut heap2 = BinaryHeap::new();
    let mut heap1 = BinaryHeap::new();

    for (char, count) in map {
        if count / 2 >= 1 {
            heap2.push((char, count / 2));
            if count % 2 != 0 {
                heap1.push(char);
            }
        } else {
            heap1.push(char);
        }
    }
    let mut ret = String::new();

    while let Some(v) = heap2.pop() {
        if ret.is_empty() && v.0 == '0' {
            continue;
        }
        for _ in 0..v.1 {
            ret.push(v.0);
        }
    }
    let str2 = ret.chars().rev().collect::<String>();
    if let Some(v) = heap1.pop() {
        ret.push(v);
    }
    ret.push_str(&str2);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            largest_palindromic("444947137".to_string()),
            "7449447".to_string()
        );
    }

    #[test]
    fn b() {
        assert_eq!(largest_palindromic("00009".to_string()), "9".to_string());
    }

    #[test]
    fn c() {
        assert_eq!(
            largest_palindromic("00011".to_string()),
            "10001".to_string()
        );
    }
}

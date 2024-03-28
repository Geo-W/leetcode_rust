use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub fn longest_valid_parentheses(s: String) -> i32 {
    let map = HashMap::from([('}', '{'), (']', '['), (')', '(')]);
    let mut vec = vec![];
    let mut ret = 0;
    let mut valid_positions = HashSet::new();
    for (idx, char) in s.chars().enumerate() {
        match vec.last() {
            None => {
                vec.push((idx, char));
            }
            Some(v) => match map.get(&char) {
                None => {
                    vec.push((idx, char));
                }
                Some(vn) => {
                    if *vn == v.1 {
                        let popped = vec.pop().unwrap();
                        valid_positions.insert(popped.0);
                        valid_positions.insert(idx);
                    } else {
                        vec.push((idx, char));
                    }
                }
            }
        }
    }
    let mut tmp = 0;
    for i in 0..s.chars().count() {
        match valid_positions.contains(&i) {
            true => {
                tmp += 1;
            }
            false => {
                ret = max(ret, tmp);
                tmp = 0;
            }
        }
    }
    ret = max(ret, tmp);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn b() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn c() {
        assert_eq!(longest_valid_parentheses("()(())".to_string()), 6);
    }
}

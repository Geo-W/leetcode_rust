/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
/// s consists of parentheses only '()[]{}'.
pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;
    let mut vec = Vec::new();
    let map = HashMap::from([('}', '{'), (']', '['), (')', '(')]);
    for char in s.chars() {
        match vec.last() {
            None => {
                vec.push(char);
            }
            Some(v) => {
                match map.get(&char) {
                    None => { vec.push(char); }
                    Some(vn) => {
                        if vn == v {
                            vec.pop();
                        } else {
                            vec.push(char);
                        }
                    }
                }
            }
        }
    }
    match vec.len() {
        0 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_valid("()".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn a() {
        let result = is_valid("{]".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn ba() {
        let result = is_valid("()[]{}".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn eba() {
        let result = is_valid("([)]".to_string());
        assert_eq!(result, false);
    }
}
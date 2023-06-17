/// Given two strings s and t, determine if they are isomorphic.
/// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
/// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
pub fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<char, char> = HashMap::new();
    let mut map2: HashMap<char, char> = HashMap::new();
    let n = s.chars().zip(t.chars());
    for (x, y) in n {
        let tmp = map.insert(x, y);
        let tmp2 = map2.insert(y, x);
        if let Some(v) = tmp {
            if v != y {
                return false;
            }
        }
        if let Some(v) = tmp2 {
            if v != x {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_isomorphic("egg".to_string(), "add".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn a() {
        let result = is_isomorphic("foo".to_string(), "bar".to_string());
        assert_eq!(result, false);
    }
}

/// Given a pattern and a string s, find if s follows the same pattern.
/// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
pub fn word_pattern(pattern: String, s: String) -> bool {
            use std::collections::HashMap;
    let mut p_to_s = HashMap::new();
    let mut s_to_p = HashMap::new();
    let (mut p, mut s) = (pattern.chars().into_iter(), s.split(" "));
    loop {
        match (p.next(),s.next()) {
            (None, None) => {
                return true;
            }
            (Some(pp), Some(ss)) => {
                match (p_to_s.get(&pp), s_to_p.get(&ss)) {
                    (None, None) => {
                        p_to_s.insert(pp, ss);
                        s_to_p.insert(ss, pp);
                    }
                    (Some(v_p), Some(v_s)) => {
                        if **v_p != *ss || *v_s != pp {
                            return false
                        }
                    }
                    _ => {
                        return false
                    }
                };
            }
            _ => {
                return false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = word_pattern("abba".to_string(), "dog cat cat dog".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn a() {
        let result = word_pattern("abba".to_string(), "dog cat cat fish".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn b() {
        let result = word_pattern("abba".to_string(), "dog dog dog dog".to_string());
        assert_eq!(result, false);
    }

        #[test]
    fn c() {
        let result = word_pattern("aaa".to_string(), "dog dog dog dog".to_string());
        assert_eq!(result, false);
    }
}

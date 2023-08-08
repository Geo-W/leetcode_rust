pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_iter = s.chars();
    let mut tmp = s_iter.next();
    for c in t.chars() {
        match tmp {
            Some(v) => {
                if c == v {
                    tmp = s_iter.next();
                }
            }
            None => {
                return true;
            }
        }
    }
    match tmp {
        Some(v) => {
            false
        }
        None => {
            true
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::standard::_0392_is_subsequence::is_subsequence;

    #[test]
    fn t1() {
        assert_eq!(is_subsequence("abc".to_string(), "ahbgdc".to_string()), true)
    }

    #[test]
    fn t2() {
        assert_eq!(is_subsequence("axc".to_string(), "ahbgdc".to_string()), false)
    }
}

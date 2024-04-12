pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
    let mut ret = String::new();
    while !(a == 0 && b == 0) {
        match (a, b) {
            (0, 1..=2) => {
                for _ in 0..b {
                    ret.push('b');
                }
                break;
            }
            (1..=2, 0) => {
                for _ in 0..a {
                    ret.push('a');
                }
                break;
            }
            _ => {
                if a > b {
                    ret.push_str("aab");
                    a -= 2;
                    b -= 1;
                } else if a < b {
                    ret.push_str("bba");
                    b -= 2;
                    a -= 1;
                } else {
                    ret.push_str("ab");
                    a -= 1;
                    b -= 1;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(str_without3a3b(1, 2), "abb".to_string());
    }

    #[test]
    fn b() {
        assert_eq!(str_without3a3b(4, 1), "aabaa".to_string());
    }

    #[test]
    fn c() {
        assert_eq!(str_without3a3b(25, 44), "".to_string());
    }
}

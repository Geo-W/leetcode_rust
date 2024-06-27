pub fn smallest_string(s: String) -> String {
    let mut r = s.into_bytes();
    let mut edited = false;
    for i in r.iter_mut() {
        if *i > 97 {
            *i -= 1;
            edited = true;
        } else if edited {
            break;
        }
    }

    if !edited {
        *r.last_mut().unwrap() = 96 + 26;
    }
    String::from_utf8(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(smallest_string("cbabc".into()), "baabc".to_string());
    }

    #[test]
    fn b() {
        assert_eq!(smallest_string("aa".to_string()), "az".to_string());
    }

    #[test]
    fn c() {
        assert_eq!(smallest_string("acbbc".to_string()), "abaab".to_string());
    }
}

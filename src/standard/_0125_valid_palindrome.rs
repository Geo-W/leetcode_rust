pub fn is_palindrome(s: String) -> bool {
    fn move_on(iterator: &mut impl Iterator<Item = (usize, u8)>) -> Option<(usize, u8)> {
        while let Some(v) = iterator.next() {
            match v.1 {
                65..=90 => {
                    return Some((v.0, v.1 + 32));
                },
                97..=122 | 48..=57 => {
                    return Some(v)
                }
                _ => {}
            }
        }
        None
    }
    let mut it_left = s.bytes().enumerate();
    let mut it_right = s.bytes().enumerate().rev();

    loop {
        let l = move_on(&mut it_left);
        let r = move_on(&mut it_right);
        // println!("comparing {:?} and {:?}", l, r);
        match (l, r) {
            (Some(l1), Some(r1)) => {
                if l1.1 != r1.1 {
                    return false;
                }
                if l1.0 >= r1.0 {
                    break;
                }
            }

            _ => {
                break;
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
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn c() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn d() {
        assert_eq!(is_palindrome("0P".to_string()), false);
    }
}

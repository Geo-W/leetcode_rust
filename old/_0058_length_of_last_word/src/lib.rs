pub fn length_of_last_word(s: String) -> i32 {
    let a = s.as_bytes();
    let len = a.len();
    let mut count = 0;
    for i in (0..len).rev() {
        match a[i] {
            32 => {
                match count {
                    0 => {}
                    _ => { break; }
                }
            }
            _ => {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn a() {
        assert_eq!(length_of_last_word(   "fly me   to   the moon  ".to_string()), 4);
    }

    #[test]
    fn b() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }

    #[test]
    fn c() {
        assert_eq!(length_of_last_word(" asdf asdasa   sss   ".to_string()), 3);
    }
}

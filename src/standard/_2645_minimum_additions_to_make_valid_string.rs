pub fn add_minimum(word: String) -> i32 {
    let mut last = "";
    let mut ret = 0;
    for char in word.chars() {
        match char {
            'a' => {
                match last {
                    "a" | "b" | "c" => ret += 2,
                    "ab" => ret += 1,
                    _ => {}
                }
                last = "a";
            }
            'b' => {
                match last {
                    "a" => {
                        last = "ab";
                    }
                    "b" | "c" => {
                        ret += 2;
                        last = "b";
                    }
                    "ab" => {
                        ret += 1;
                        last = "b";
                    }
                    _ => {
                        last = "b"
                    }
                }
            }
            'c' => {
                match last {
                    "ab" => {
                        last = "";
                    }
                    "a" => {
                        ret += 1;
                        last = "";
                    }
                    "b" => {
                        ret += 1;
                        last = "";
                    }
                    "c" => {
                        ret += 2;
                        last = "c";
                    }
                    _ => {
                        last = "c";
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    match last {
        "ab" => {
            ret += 1;
        }
        "a" | "b" | "c" => {
            ret += 2;
        }
        _ => {}
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(add_minimum("b".to_string()), 2);
    }

    #[test]
    fn b() {
        assert_eq!(add_minimum("aaa".to_string()), 6);
    }

    #[test]
    fn c() {
        assert_eq!(add_minimum("abc".to_string()), 0);
    }
}

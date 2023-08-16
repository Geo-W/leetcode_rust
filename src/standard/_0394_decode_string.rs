/// Given an encoded string, return its decoded string.
/// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
pub fn decode_string(s: String) -> String {
    fn parse_parenthesis(num: &str, char: &mut std::str::Chars) -> String {
        let mut new_num = String::new();
        let mut tmp = String::new();
        while let Some(v) = char.next() {
            match v {
                '0'..='9' => {
                    new_num.push(v);
                }
                '[' => {
                    tmp.push_str(&parse_parenthesis(&new_num, char));
                    new_num.clear();
                }
                ']' => {
                    return tmp.repeat(num.parse::<usize>().unwrap());
                }
                _ => {
                    tmp.push(v);
                }
            }
        }
        tmp.repeat(num.parse::<usize>().unwrap())
    }

    parse_parenthesis("1", &mut s.chars())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
    }

    #[test]
    fn t2() {
        assert_eq!(
            decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string()),
            "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string());
    }
}
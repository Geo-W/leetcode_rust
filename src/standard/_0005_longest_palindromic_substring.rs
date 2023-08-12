/// Return the longest palindrome substring in a string
pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut target_idx = 0;
    let mut target_i: usize = 0;
    let mut target_idx2 = 0;
    let mut target_i2: usize = 0;
    for idx in 0..len {
        let mut i = 1;
        while idx.checked_sub(i).is_some() && idx + i < len {
            if s[idx - i] == s[idx + i] {
                i += 1;
                if i - 1 > target_i {
                    target_i = i - 1;
                    target_idx = idx;

                }
            } else {
                break;
            }
        }
        let mut i = 1;
        while idx.checked_sub(i).is_some() {
            if s[idx - i] == s[idx+i-1] {
                i += 1;
                if i - 1 > target_i2 {
                    target_i2 = i - 1;
                    target_idx2 = idx;
                }
            } else {
                break;
            }
        }
    }

    if target_i >= target_i2 {
        s[(target_idx - target_i)..=(target_idx + target_i)].iter().collect()
    } else {
        s[(target_idx2 - target_i2)..=(target_idx2 + target_i2-1)].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = longest_palindrome("babad".to_string());
        assert_eq!(result, "bab".to_string());
    }

    #[test]
    fn a() {
        let result = longest_palindrome("cbbd".to_string());
        assert_eq!(result, "bb".to_string());
    }

    #[test]
    fn b() {
        let res = longest_palindrome("tattarrattat".to_string());
        assert_eq!(res, "tattarrattat".to_string());
    }
}

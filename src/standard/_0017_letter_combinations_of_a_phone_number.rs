/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number in phone button could represent.
pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut ret = vec![];
    let mut tmp = String::new();

    fn recur(ptr: usize, ret: &mut Vec<String>, tmp: &mut String, max_len: usize, digits: &String) {
        if tmp.len() == max_len {
            ret.push(tmp.clone());
            return;
        }
        let cur_str = get_numbers(&digits[ptr..=ptr]);
        for k in 0..cur_str.len() {
            tmp.push_str(&cur_str[k..=k]);
            recur(ptr + 1, ret, tmp, max_len, digits);
            tmp.pop();
        }
    }
    if !digits.is_empty() {
        recur(0, &mut ret, &mut tmp, digits.len(), &digits);
    }
    ret
}

#[inline]
fn get_numbers(c: &str) -> &'static str {
    match c {
        "1" => "",
        "2" => "abc",
        "3" => "def",
        "4" => "ghi",
        "5" => "jkl",
        "6" => "mno",
        "7" => "pqrs",
        "8" => "tuv",
        "9" => "wxyz",
        "0" => "",
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec_string!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn b() {
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
    }
}

use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
    fn count_chars(word: String) -> (Vec<char>, Vec<i32>) {
        let mut char_count = HashMap::new();

        for c in word.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let mut chars = char_count.keys().copied().collect::<Vec<char>>();
        let mut counts = char_count.values().copied().collect::<Vec<i32>>();

        chars.sort();
        counts.sort();

        (chars, counts)
    }

    let (chars1, counts1) = count_chars(word1);
    let (chars2, counts2) = count_chars(word2);

    chars1 == chars2 && counts1 == counts2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(close_strings("abc".to_string(), "bca".to_string()), true);
    }

    #[test]
    fn b() {
        assert_eq!(close_strings("a".to_string(), "aa".to_string()), false);
    }

    #[test]
    fn c() {
        assert_eq!(
            close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
    }
}

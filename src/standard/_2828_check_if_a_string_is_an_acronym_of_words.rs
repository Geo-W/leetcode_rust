pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    s == words
        .into_iter()
        .map(|x| x.chars().next().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            is_acronym(vec_string!["alice", "bob", "charlie"], "abc".to_string()),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            is_acronym(vec_string!["an", "apple"], "a".to_string()),
            false
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            is_acronym(
                vec_string!["never", "gonna", "give", "up", "on", "you"],
                "ngguoy".to_string()
            ),
            true
        );
    }
}

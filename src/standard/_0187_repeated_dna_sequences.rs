use std::collections::HashSet;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }
    let mut map = HashSet::new();
    let mut ret2 = HashSet::new();
    for i in 0..s.len() - 10 + 1 {
        let slice = &s[i..i + 10];
        if map.get(&slice).is_some() {
            ret2.insert(slice);
        } else {
            map.insert(slice);
        }
    }
    ret2.into_iter().map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use crate::vec_string;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
            vec_string!["AAAAACCCCC", "CCCCCAAAAA"]
        );
    }
    #[test]
    fn b() {
        assert_eq!(
            find_repeated_dna_sequences("AAAAAAAAAAA".to_string()),
            vec_string!["AAAAAAAAAAA"]
        );
    }
}

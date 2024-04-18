use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub fn get_hint(secret: String, guess: String) -> String {
    let mut bulls = 0;
    let mut cows = 0;

    let mut unfound_map = HashMap::new();

    let mut map = secret
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, x)| {
            acc.entry(x).or_insert_with(HashSet::new).insert(idx);
            return acc;
        });

    for (idx, c) in guess.chars().enumerate() {
        if let Some(v) = map.get_mut(&c) {
            if v.contains(&idx) {
                bulls += 1;
                v.remove(&idx);
            } else {
                *unfound_map.entry(c).or_insert(0) += 1;
            }
        }
    }

    for (c, count) in unfound_map {
        if let Some(v) = map.get(&c) {
            cows += min(v.len(), count);
        }
    }

    format!("{bulls}A{}B", cows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            get_hint("1807".to_string(), "7810".to_string()),
            "1A3B".to_string()
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            get_hint("1122".to_string(), "2211".to_string()),
            "0A4B".to_string()
        );
    }
}

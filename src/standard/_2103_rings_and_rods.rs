use std::collections::HashMap;

pub fn count_points(rings: String) -> i32 {
    let mut map = HashMap::new();
    for i in (0..rings.len()).step_by(2) {
        let a = map.entry(&rings[i + 1..i + 2]).or_insert(0);
        let num = match &rings[i..i + 1] {
            "R" => 1,
            "G" => 2,
            "B" => 4,
            _ => unimplemented!(),
        };
        if *a ^ num > *a {
            *a ^= num;
        }
    }
    map.iter().map(|(_, &x)| if x == 7 { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(count_points("B0B6G0R6R0R6G9".to_string()), 1);
    }

    #[test]
    fn b() {
        assert_eq!(count_points("B0R0G0R9R0B0G0".to_string()), 1);
    }

    #[test]
    fn c() {
        assert_eq!(count_points("G4".to_string()), 0);
    }
}

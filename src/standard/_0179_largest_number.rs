use std::cmp::Ordering;

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut ar = nums
        .into_iter()
        .map(|x| Num(x.to_string()))
        .collect::<Vec<_>>();
    ar.sort();
    ar.reverse();
    if ar.first() == Some(&Num("0".to_string())) {
        return "0".to_string();
    }
    ar.into_iter().map(|x| x.0).collect::<Vec<_>>().join("")
}


#[derive(Debug)]
struct Num(String);

impl Eq for Num {}

impl PartialEq<Self> for Num {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<Self> for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // here comparing it as string, converting it to i64/i128 and compare with number is more efficient.
        format!("{}{}", self.0, other.0).partial_cmp(&format!("{}{}", other.0, self.0))
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(largest_number(vec![10, 2]), 210.to_string());
    }

    #[test]
    fn b() {
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330".to_string());
    }

    #[test]
    fn c() {
        assert_eq!(largest_number(vec![34323, 3432]), 343234323.to_string());
    }

    #[test]
    fn d() {
        assert_eq!(largest_number(vec![432, 43243]), 43243432.to_string());
    }
}

/// Given two strings needle and haystack,
/// return the index of the first occurrence of needle in haystack,
/// or -1 if needle is not part of haystack.
pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(v) => v as i32,
        None => -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            str_str("fdsasd".to_string(), "sas".to_string()),2
        )
    }

}

pub fn title_to_number(column_title: String) -> i32 {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    for (m, n) in (65..=90).zip(1..=26) {
        map.insert(m, n);
    }
    let bytes = column_title.as_bytes();
    let len = column_title.len();
    let mut count = 0;
    let mut result = 0;
    for i in (0..len).rev() {
        result += (map.get(&(bytes[i])).unwrap() * 26_i32.pow(count));
        count += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(title_to_number("A".to_string()), 1);
    }

    #[test]
    fn a() {
        assert_eq!(title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn b() {
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }
}

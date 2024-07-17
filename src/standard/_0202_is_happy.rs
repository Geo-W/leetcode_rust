pub fn is_happy(n: i32) -> bool {
    use std::collections::HashSet;
    let mut n = n;
    let mut map = HashSet::new();
    let mut tmp = 0;

    while n != 1 {
        if map.contains(&n) {
            return false;
        }
        map.insert(n);
        while n > 0 {
            tmp += (n % 10).pow(2);
            n /= 10;
        }
        n = tmp;
        tmp = 0;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(is_happy(19), true);
    }

    #[test]
    fn b() {
        assert_eq!(is_happy(2), false);
    }
}

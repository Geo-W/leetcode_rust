use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    fn re(map: &mut HashMap<i32, i32>, n: i32) -> i32 {
        return if let Some(v) = map.get(&n) {
            *v
        } else {
            let ret = re(map, n - 2) + re(map, n - 1);
            map.insert(n, ret);
            ret
        };
    }
    re(&mut HashMap::from_iter([(1, 1), (2, 2)]), n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn b() {
        assert_eq!(climb_stairs(42), 433494437);
    }
}

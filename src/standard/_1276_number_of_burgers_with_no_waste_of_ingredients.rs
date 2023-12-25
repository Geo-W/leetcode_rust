pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let x = (tomato_slices - 2 * cheese_slices) / 2;
    let y = cheese_slices - x;
    if 4 * x + 2 * y != tomato_slices || x + y != cheese_slices || x < 0 || y < 0 {
        return vec![];
    }
    vec![x, y]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
    }

    #[test]
    fn b() {
        assert_eq!(num_of_burgers(17, 4), vec![]);
    }

    #[test]
    fn c() {
        assert_eq!(num_of_burgers(4, 17), vec![]);
    }
}

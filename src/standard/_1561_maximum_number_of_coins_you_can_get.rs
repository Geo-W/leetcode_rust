pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort();
    piles
        .iter()
        .rev()
        .skip(1)
        .step_by(2)
        .take(piles.len() / 3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
    }

    #[test]
    fn b() {
        assert_eq!(max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}

/// Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.
pub fn sum_of_multiples(n: i32) -> i32 {
    (0..=n).filter(|x| *x % 3 == 0 || *x % 5 == 0 || *x % 7 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(sum_of_multiples(7), 21);
    }
    
    #[test]
    fn b() {
        assert_eq!(sum_of_multiples(10), 40);
    }

    #[test]
    fn c() {
        assert_eq!(sum_of_multiples(9), 30);
    }
}
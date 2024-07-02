pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    fn check_prime(num: i32) -> bool {
        let a = (num as f64).sqrt() as i32;
        for i in 2..=a {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }
    let mut first_idx = 0;
    let mut last_idx = nums.len() - 1;
    for (idx, &num) in nums.iter().enumerate() {
        if check_prime(num) {
            first_idx = idx;
            break;
        }
    }
    for (idx, &num) in nums.iter().enumerate().rev() {
        if check_prime(num) {
            last_idx = idx;
            break;
        }
    }
    (last_idx - first_idx) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
    }
}

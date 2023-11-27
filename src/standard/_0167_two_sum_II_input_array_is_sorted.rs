pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    loop {
        match numbers[(left + 1)..=right].binary_search(&(target - numbers[left])) {
            Ok(v) => {
                return vec![(left + 1) as i32, (v + left + 2) as i32];
            }
            Err(v) => {
                right = v + left;
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn b() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn c() {
        assert_eq!(two_sum(vec![-1, 0], -1), [1, 2]);
    }

    #[test]
    fn d() {
        assert_eq!(two_sum(vec![2, 4, 7, 11], 15), vec![2, 4]);
    }
}

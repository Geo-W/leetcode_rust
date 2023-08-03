///Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len() + 1;
    let mut sum: i32 = ((len * (len - 1)) / 2) as i32;
    for i in nums {
        sum = sum - i;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn a() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn b() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}

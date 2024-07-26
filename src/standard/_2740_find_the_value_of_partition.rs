pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums.windows(2).map(|x| x[1] - x[0]).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_value_of_partition(vec![1, 3, 2, 4]), 1);
    }

    #[test]
    fn b() {
        assert_eq!(find_value_of_partition(vec![100, 1, 10]), 9);
    }
}

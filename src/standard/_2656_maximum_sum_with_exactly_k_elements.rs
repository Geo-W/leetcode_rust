/// You are given a 0-indexed integer array nums and an integer k. Your task is to perform the following operation exactly k times in order to maximize your score:
/// Select an element m from nums.
/// Remove the selected element m from the array.
/// Add a new element with a value of m + 1 to the array.
/// Increase your score by m.
/// Return the maximum score you can achieve after performing the operation exactly k times.
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().max().unwrap() * k + ((k as f32 - 1.0) * (k as f32 / 2.0)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
    }

    #[test]
    fn b() {
        assert_eq!(maximize_sum(vec![5, 5, 5], 2), 11);
    }
}

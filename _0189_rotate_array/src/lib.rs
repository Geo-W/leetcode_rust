/// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let drift = k as usize % nums.len();
    nums.extend_from_within(0..=drift);
    for i in 0..len {
        nums[i] = nums[len - drift + i]
    }
    nums.truncate(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut vec, 3);
        assert_eq!(vec, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn a() {
        let mut vec = vec![-1, -100, 3, 99];
        rotate(&mut vec, 2);
        assert_eq!(vec, vec![3, 99, -1, -100]);
    }

    #[test]
    fn b() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        rotate(&mut vec, 1);
        assert_eq!(vec, vec![2, 3, 4, 5, 6, 1]);
    }
}

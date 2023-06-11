pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    while left <= right {
        let current: usize = ((right - left) / 2 + left) as usize;
        if nums[current] > target {
            right = current as i32 - 1;
        } else if nums[current] < target {
            left = current as i32 + 1;
        } else if nums[current] == target {
            return current as i32;
        }
    }
    return -1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = binary_search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(result, 4)
    }

    #[test]
    fn b() {
        let result = binary_search(vec![-1, 0, 3, 5, 12], 9);
        assert_eq!(result, -1)
    }

    #[test]
    fn e() {
        let result = binary_search(vec![-1, 0, 3, 9, 12], 9);
        assert_eq!(result, 3)
    }

    #[test]
    fn f() {
        let result = binary_search(vec![5], 1);
        assert_eq!(result, -1)
    }

    #[test]
    fn g() {
        let result = binary_search(vec![-1, 0, 3, 5, 9, 12], 3);
        assert_eq!(result, 2)
    }
}



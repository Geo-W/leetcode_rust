#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dd() {
        let result = search_range(vec![5, 7, 7, 8, 8, 10], 8);
    }

    #[test]
    fn a() {
        let result = search_range(vec![5, 7, 7, 8, 8, 8, 8, 8, 8, 8, 10], 8);
        assert_eq!(result, vec![3, 9])
    }

    #[test]
    fn b() {
        let result = search_range(vec![1], 1);
        assert_eq!(result, vec![0, 0])
    }
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result_left: i32 = -1;
    let mut result_right: i32 = -1;

    // find left boundary
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    while left <= right {
        let current: usize = ((right - left) / 2 + left) as usize;
        if nums[current] > target {
            right = current as i32 - 1;
        } else if nums[current] < target {
            left = current as i32 + 1;
        } else if nums[current] == target {
            if current == 0 || nums[current - 1] < target {
                result_left = current as i32;
                break;
            } else {
                right = current as i32 - 1;
            }
        }
    }

    // find right boundary
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    while left <= right {
        let current: usize = ((right - left) / 2 + left) as usize;
        if nums[current] > target {
            right = current as i32 - 1;
        } else if nums[current] < target {
            left = current as i32 + 1;
        } else if nums[current] == target {
            println!("{:?},{:?}", current, nums.len());
            if current +1 == nums.len() || nums[current + 1] > target {
                result_right = current as i32;
                break;
            } else {
                left = current as i32 + 1;
            }
        }
    }

    return vec![result_left as i32, result_right as i32];
}
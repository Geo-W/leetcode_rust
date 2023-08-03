pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    let mut current: usize = 0;
    while left < right {
        current = ((right - left) / 2 + left) as usize;
        println!("{:?}", current);
        if nums[current] > target {
            right = current as i32 - 1;
        } else if nums[current] < target {
            left = current as i32 + 1;
        } else if nums[current] == target {
            return current as i32;
        }
    }
    if left == right || left < 0 || right < 0 {
        if target > nums[left as usize] {
            return left + 1;
        } else {
            return left;
        }
    }
    return (current + 1) as i32;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn b() {
        let result = search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn c() {
        let result = search_insert(vec![1, 3, 5, 6], 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn d() {
        let result = search_insert(vec![1, 3], 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn e() {
        let result = search_insert(vec![3, 5, 7, 9, 10], 8);
        assert_eq!(result, 3);
    }
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    let mut current: usize = 0;

    while left < right {
        current = ((right - left) / 2 + left) as usize;
        println!("current{:?}", current);
        if nums[current] < target {
            left = current as i32 + 1; //3
            println!("left:{:?}, right:{:?}", left, right);
        } else if nums[current] > target {
            right = current as i32 - 1; // 3
            println!("left:{:?}, right:{:?}", left, right);
        } else if nums[current] == target {
            return current as i32;
        }
    }

    if left >= right || left < 0 || right < 0 {
        if nums[left as usize] >= target {
            return left;
        } else {
            return left + 1;
        }
    } else {
        return 0;
    }
}
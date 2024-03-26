pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut ret = usize::MAX;
    let mut ptr_low = 0;
    let mut ptr_up = 0;
    let mut cur = nums[0];
    loop {
        if cur < target {
            ptr_up += 1;
            if ptr_up > nums.len() - 1 {
                break;
            }
            cur += nums[ptr_up];
        } else {
            ret = std::cmp::min(ptr_up - ptr_low + 1, ret);
            cur -= nums[ptr_low];
            ptr_low += 1;
        }
    }

    if ret == usize::MAX {
        0
    } else {
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn b() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn c() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}

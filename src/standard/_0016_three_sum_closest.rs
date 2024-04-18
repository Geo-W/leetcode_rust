pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut ret = 0;
    let mut ret_deviation = i32::MAX;

    let mut last = i32::MAX;
    let mut last_left = i32::MAX;
    let mut last_right = i32::MAX;

    let mut idx_3 = nums.len() - 1;
    let mut idx_2;

    for idx in 0..nums.len() - 2 {
        if nums[idx] == last {
            continue;
        }
        last = nums[idx];
        // initialize pointers for left and right
        idx_3 = nums.len() - 1;
        idx_2 = idx + 1;
        while idx_3 > idx_2 {
            let tmp = nums[idx] + nums[idx_2] + nums[idx_3];
            // if sum > 0, move the right pointer; also check if it is equal to last calculated element.
            if tmp > target || last_right == nums[idx_3] {
                if (tmp - target).abs() < ret_deviation {
                    ret = tmp;
                    ret_deviation = (tmp - target).abs()
                }

                last_right = nums[idx_3];
                idx_3 -= 1;
                continue;
            }
            if tmp < target || last_left == nums[idx_2] {
                if (tmp - target).abs() < ret_deviation {
                    ret = tmp;
                    ret_deviation = (tmp - target).abs()
                }

                last_left = nums[idx_2];
                idx_2 += 1;
                continue;
            }
            if tmp == target {
                return target;
            }
        }
        last_right = i32::MAX;
        last_left = i32::MAX;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn b() {
        assert_eq!(three_sum_closest(vec![0, 1, 2], 3), 3);
    }
}

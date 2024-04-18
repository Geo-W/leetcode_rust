pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let mut pos = 1;
    while pos < nums.len() + 1 {
        // swap until current position got the right number, like [4,1,2,3] -> [1,2,3,4].
        // if one of the number is not presented, that position will get another number.
        'inner: while nums[pos - 1] != pos as i32 {
            // if the number is out of the scope, leave it there. Say, 7 out of [1,7,3,2]
            if nums[pos - 1] > 0 && nums[pos - 1] <= nums.len() as i32 {
                let target_num = nums[pos - 1];
                let target_idx = target_num as usize - 1;
                nums.swap(pos - 1, target_idx);
                if nums[pos - 1] == nums[target_idx] {
                    break 'inner;
                }
            } else {
                break 'inner;
            }
        }
        pos += 1;
    }

    for (idx, num) in nums.iter().enumerate() {
        if idx as i32 + 1 != *num {
            return idx as i32 + 1;
        }
    }

    nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn c() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn d() {
        assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
    }

    #[test]
    fn e() {
        assert_eq!(first_missing_positive(vec![1, 1]), 2);
    }
}

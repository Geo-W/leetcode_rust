use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut ret = 0;

    let mut map = std::collections::HashMap::new();

    let nums = nums.into_iter().collect::<HashSet<i32>>();

    for num in nums {
        let mut left_length = 0;
        let mut right_length = 0;

        let mut left = num - 1;
        let mut right = num + 1;

        while map.contains_key(&left) {
            left_length += 1;
            left -= 1;
        }
        while map.contains_key(&right) {
            right_length += 1;
            right += 1;
        }
        map.insert(num, 1);
        ret = std::cmp::max(ret, left_length + right_length + 1);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}

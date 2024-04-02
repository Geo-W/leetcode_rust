use std::collections::{HashMap, HashSet};

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut ret = vec![];

    let mut last = i32::MAX;
    let mut last_left = i32::MAX;
    let mut last_right = i32::MAX;

    let mut idx_3 = nums.len() - 1;
    let mut idx_2;

    for idx in 0..nums.len() - 2 {
        if nums[idx] > 0 {
            break;
        }
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
            if tmp > 0 || last_right == nums[idx_3] {
                last_right = nums[idx_3];
                idx_3 -=1;
                continue;
            }
            if tmp < 0 || last_left == nums[idx_2] {
                last_left = nums[idx_2];
                idx_2 +=1;
                continue;
            }
            if tmp == 0 {
                ret.push(
                    vec![nums[idx], nums[idx_2], nums[idx_3]]
                );
                last_left = nums[idx_2];
                idx_2 += 1;
            }
        }
        last_right = i32::MAX;
        last_left = i32::MAX;
    }

    ret
}

pub fn three_sum2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    let mut map = HashMap::new();
    let mut used_fixed_num = HashSet::new();
    let mut used_inner_num = HashSet::new();
    for idx in 0..nums.len() {
        if used_fixed_num.contains(&nums[idx]) {
            continue;
        }
        for idx_2 in (idx + 1)..nums.len() {
            if let Some(v) = map.get(&(0 - nums[idx] - nums[idx_2])) {
                if !used_fixed_num.contains(&nums[idx_2])
                    && !used_fixed_num.contains(&(0 - nums[idx] - nums[idx_2]))
                    && !used_inner_num.contains(&nums[idx_2])
                {
                    used_inner_num.insert(nums[idx_2]);
                    ret.push(vec![nums[idx], 0 - nums[idx] - nums[idx_2], nums[idx_2]]);
                };
            } else {
                map.insert(nums[idx_2], 1);
            }
        }
        used_fixed_num.insert(nums[idx]);
        map.clear();
        used_inner_num.clear();
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec_vec![[-1, -1, 2], [-1, 0, 1]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec_vec![[0, 0, 0]]);
    }

    #[test]
    fn c() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]),
            vec_vec![
                [-4, 0, 4],
                [-4, 1, 3],
                [-3, -1, 4],
                [-3, 0, 3],
                [-3, 1, 2],
                [-2, -1, 3],
                [-2, 0, 2],
                [-1, -1, 2],
                [-1, 0, 1]
            ]
        );
    }

    #[test]
    fn d() {
        assert_eq!(three_sum(vec![3, -2, 1, 0]), Vec::<Vec<i32>>::new());
    }
}

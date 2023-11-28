use std::collections::HashSet;

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    fn recur(ret: &mut Vec<Vec<i32>>, ptr: usize, tmp: &mut Vec<i32>, last: i32, nums: &Vec<i32>) {
        if tmp.len() > 1 {
            ret.push(tmp.clone());
        }
        let mut set = HashSet::new();
        for i in ptr..nums.len() {
            if nums[i] >= last {
                if set.insert(nums[i]) {
                    tmp.push(nums[i]);
                    recur(ret, i + 1, tmp, nums[i], nums);
                    tmp.pop();
                }
            }
        }
    }
    recur(&mut ret, 0, &mut tmp, -101, &nums);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_subsequences(vec![4, 6, 7, 7]),
            vec_vec![
                [4, 6],
                [4, 6, 7],
                [4, 6, 7, 7],
                [4, 7],
                [4, 7, 7],
                [6, 7],
                [6, 7, 7],
                [7, 7]
            ]
        );
    }

    #[test]
    fn b() {
        assert_eq!(find_subsequences(vec![4, 4, 3, 2, 1]), vec_vec![[4, 4]]);
    }
}

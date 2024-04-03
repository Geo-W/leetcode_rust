pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ret = std::collections::HashSet::new();
    let mut pos = 1;
    while pos < nums.len() + 1 {
        'inner: while nums[pos - 1] != pos as i32 {
            let target_num = nums[pos - 1];
            let target_idx = target_num as usize - 1;
            if nums[pos - 1] == nums[target_idx] {
                ret.insert(nums[pos - 1]);
                break 'inner;
            }
            nums.swap(pos - 1, target_idx);
        }
        pos += 1;
    }
    ret.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![3, 2]);
    }

    #[test]
    fn b() {
        assert_eq!(find_duplicates(vec![1, 1]), vec![1]);
    }
}

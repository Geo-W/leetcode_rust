pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    fn recur(ret: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, nums: &Vec<i32>, cur: usize, max: usize) {
        ret.push(tmp.clone());
        for i in cur..max {
            tmp.push(nums[i]);
            recur(ret, tmp, nums, i + 1, max);
            tmp.pop();
        }
    }
    recur(&mut ret, &mut tmp, &nums, 0, nums.len());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut rhs = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let mut lhs = subsets(vec![1, 2, 3]);
        rhs.sort();
        lhs.sort();
        assert_eq!(lhs, rhs);
    }
}
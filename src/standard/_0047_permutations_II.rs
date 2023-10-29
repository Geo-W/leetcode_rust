use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = HashSet::new();
    let mut tmp = vec![];
    fn rec(tmp: &mut Vec<i32>, nums: &Vec<i32>, len: usize, ret: &mut HashSet<Vec<i32>>) {
        if tmp.len() == len {
            ret.insert(tmp.clone());
        } else {
            for (idx, item) in nums.iter().enumerate() {
                tmp.push(*item);
                rec(
                    tmp,
                    &nums
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|(_, n)| *n)
                        .collect(),
                    len,
                    ret,
                );
                tmp.pop();
            }
        }
    }
    rec(&mut tmp, &nums, nums.len(), &mut ret);
    ret.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut lhs = permute_unique(vec![1, 1, 2]);
        lhs.sort();
        let mut rhs = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        rhs.sort();

        assert_eq!(lhs, rhs);
    }
}

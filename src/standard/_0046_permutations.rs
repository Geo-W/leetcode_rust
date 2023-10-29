pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    fn rec(tmp: &mut Vec<i32>, nums: &Vec<i32>, len: usize, ret: &mut Vec<Vec<i32>>) {
        if tmp.len() == len {
            ret.push(tmp.clone());
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
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

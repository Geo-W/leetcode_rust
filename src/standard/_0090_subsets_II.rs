pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    nums.sort();
    // let mut set = HashSet::new();
    let mut last = -11;
    fn recur(
        ret: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
        nums: &Vec<i32>,
        cur: usize,
        max: usize,
        last: &mut i32,
    ) {
        ret.push(tmp.clone());
        for i in cur..max {
            // let t = set.insert(nums[i]);
            if nums[i] != *last {
                tmp.push(nums[i]);
                recur(ret, tmp, nums, i + 1, max, last);
                tmp.pop();
            }
            *last = nums[i];
        }
    }
    recur(&mut ret, &mut tmp, &nums, 0, nums.len(), &mut last);
    ret
}

#[cfg(test)]
mod tests {
    use crate::vec_vec;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            subsets_with_dup(vec![1, 2, 2]),
            vec_vec![[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(subsets_with_dup(vec![0]), vec_vec![[], [0]]);
    }

    #[test]
    fn c() {
        assert_eq!(
            subsets_with_dup(vec![4, 4, 4, 1, 4]),
            vec_vec![
                [],
                [1],
                [1, 4],
                [1, 4, 4],
                [1, 4, 4, 4],
                [1, 4, 4, 4, 4],
                [4],
                [4, 4],
                [4, 4, 4],
                [4, 4, 4, 4]
            ]
        );
    }
}

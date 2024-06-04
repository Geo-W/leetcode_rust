use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    fn rob1(nums: &[i32]) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut ret = vec![];
        ret.push(nums[0]);
        ret.push(max(nums[0], nums[1]));
        for idx in 2..nums.len() {
            ret.push(max(ret[idx - 2] + nums[idx], ret[idx - 1]));
        }
        *ret.last().unwrap_or(&0)
    }
    if nums.len() == 1 {
        return nums[0];
    } else {
        max(rob1(&nums[0..nums.len() - 1]), rob1(&nums[1..nums.len()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn c() {
        assert_eq!(rob(vec![1]), 1);
    }
}

use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
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

// [2,7,9,3,1]
//f(1) = 1
//f(2) = 1 or 2
//f(3) = max(f(1)+3, f(2))
//f(4) = max(f(2)+4, f(3))

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn b() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}

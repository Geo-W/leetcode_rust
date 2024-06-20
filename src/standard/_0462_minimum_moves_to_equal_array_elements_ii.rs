pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mid = if nums.len() % 2 == 1 {
        nums[nums.len() / 2]
    } else {
        (nums[nums.len() / 2] + nums[nums.len() / 2 - 1]) / 2
    };

    nums.into_iter().map(|x| (mid - x).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(min_moves2(vec![1, 2, 3]), 2);
    }

    #[test]
    fn b() {
        assert_eq!(min_moves2(vec![1, 10, 2, 9]), 16);
    }

    #[test]
    fn c() {
        assert_eq!(min_moves2(vec![3, 0, 6, 2, 4, 7, 0, 0]), 18);
    }
}

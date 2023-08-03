/// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
/// Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where
/// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut counter = 0;
    if nums.len() == 0 {
        return 0
    }
    while index + (nums[index] as usize) < (nums.len() - 1) {
        counter +=1;
        match &nums[index + 1..=(index + nums[index] as usize)]
            .iter()
            .enumerate()
            .max_by(|&(enum1, a), &(enum2, b)| {
                ((*a as usize) + enum1).cmp(&((*b as usize) + enum2))
            })
            .map(|(index, _)| index) {
            Some(v) => {
                index = index + 1 + v;
            }
            None => {
                panic!("cannot jump to last");
            }
        }
    };
    counter+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }


    #[test]
    fn a() {
        assert_eq!(jump(vec![0]), 0);
    }


    #[test]
    fn b() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }
}

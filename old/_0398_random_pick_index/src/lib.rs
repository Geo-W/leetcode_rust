use rand::seq::SliceRandom;
use std::collections::HashMap;

/// Given an integer array nums with possible duplicates, randomly output the index of a given target number. You can assume that the given target number must exist in the array.
/// Implement the Solution class:
///     Solution(int[] nums) Initializes the object with the array nums.
///     int pick(int target) Picks a random index i from nums where nums[i] == target. If there are multiple valid i's, then each index should have an equal probability of returning.
struct Solution {
    map: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        nums.iter()
            .enumerate()
            .for_each(|(idx, item)| match map.get_mut(item) {
                None => {
                    map.insert(*item, vec![idx as i32]);
                }
                Some(v) => {
                    v.push(idx as i32);
                }
            });
        Solution { map }
    }

    fn pick(&self, target: i32) -> i32 {
        let vec = self.map.get(&target).unwrap();
        *vec.choose(&mut rand::thread_rng()).unwrap()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let obj = Solution::new(vec![1, 2, 3, 3, 3]);
        assert_eq!(obj.pick(3) >= 2 && obj.pick(3) <= 4, true);
        assert_eq!(obj.pick(1), 0);
        assert_eq!(obj.pick(3) >= 2 && obj.pick(3) <= 4, true);
    }
}

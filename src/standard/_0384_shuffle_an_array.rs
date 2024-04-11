use rand::seq::SliceRandom;
use rand::thread_rng;

struct Solution {
    init: Vec<i32>,
    stat: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            init: nums.clone(),
            stat: nums,
        }
    }

    fn reset(&self) -> Vec<i32> {
        self.init.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        self.stat.shuffle(&mut rng);
        self.stat.clone()
    }
}

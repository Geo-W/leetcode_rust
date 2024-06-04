use std::collections::HashSet;

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut steps = vec![0; target as usize + 1];
    let set = nums.iter().map(|x| *x).collect::<HashSet<_>>();
    for idx in 0..steps.len() {
        let mut tmp = 0;
        let idx = idx as i32;
        if set.contains(&idx) {
            tmp += 1;
        }
        for &num in &nums {
            if idx - num >= 0 && steps[(idx - num) as usize] != 0 {
                tmp += steps[(idx - num) as usize];
            }
        }
        steps[idx as usize] = tmp;
    }


    *steps.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn b() {
        assert_eq!(combination_sum4(vec![9], 3), 0);
    }
}

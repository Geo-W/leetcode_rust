/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
pub fn single_number(nums: Vec<i32>) -> i32{
    nums.iter().fold(0, |cur, next| cur ^ next)
}

/// this func use hashset extra space, does not comply with the question
pub fn single_number_with_hash(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for i in nums.iter() {
        if set.contains(&i) == false {
            set.insert(i);
        } else {
            set.remove(&i);
        }
    }
    let result = *(set.into_iter().next().unwrap());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(single_number(vec![2,2,1]), 1);
    }

    #[test]
    fn a() {
        assert_eq!(single_number(vec![4,1,2,1,2]), 4);
    }

    #[test]
    fn b() {
        assert_eq!(single_number(vec![1]), 1);

    }
}

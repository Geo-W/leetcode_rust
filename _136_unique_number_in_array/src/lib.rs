pub fn single_number(nums: Vec<i32>) -> i32 {
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

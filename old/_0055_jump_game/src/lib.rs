/// You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
/// Return true if you can reach the last index, or false otherwise.
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut index = 0;
    while index + (nums[index] as usize) < (nums.len() -1) {
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
                return false;
            }
        }
    };
    true
}


pub fn can_jump2(nums: Vec<i32>) -> bool {
    let len = nums.len() - 1;
    let mut index = 0;
    loop {
        if index + nums[index] as usize >= len {
            return true;
        } else if nums[index] == 0 {
            return false;
        } else {
            let max_index = &nums[index + 1..=(index + nums[index] as usize)]
                .iter()
                .enumerate()
                .max_by(|&(enum1, a), &(enum2, b)| {
                    ((*a as usize) + enum1).cmp(&((*b as usize) + enum2))
                })
                .map(|(index, _)| index)
                .unwrap();
            index = max_index + index + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn a() {
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn b() {
        assert_eq!(can_jump(vec![0]), true);
    }

    #[test]
    fn c() {
        assert_eq!(can_jump(vec![2, 0]), true);
    }

    #[test]
    fn d() {
        assert_eq!(can_jump(vec![2, 0, 1, 0, 1]), false);
    }

    #[test]
    fn e() {
        assert_eq!(can_jump(vec![4, 2, 0, 0, 1, 1, 4, 4, 4, 0, 4, 0]), true);
    }
}

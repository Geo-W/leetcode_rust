pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut ptr = 0;
    let mut even = true;
    let mut ret = 0;
    while ptr < nums.len() - 1 {
        if even {
            if nums[ptr] == nums[ptr + 1] {
                ret += 1;
            } else {
                even = !even;
            }
        } else {
            even = !even;
        }
        ptr += 1;
    }
    if even {
        ret += 1;
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(min_deletion(vec![1, 1, 2, 3, 5]), 1);
    }

    #[test]
    fn b() {
        assert_eq!(min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
    }

    #[test]
    fn c() {
        assert_eq!(min_deletion(vec![8]), 1);
    }
}
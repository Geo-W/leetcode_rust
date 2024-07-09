pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut one_s = 0;
    let mut last_one_s = 0;
    let mut have_zero = false;
    for num in nums {
        match num {
            0 => {
                ret = std::cmp::max(ret, one_s + last_one_s);
                last_one_s = one_s;
                one_s = 0;
                have_zero = true;
            }
            1 => {
                one_s += 1;
            }
            _ => unreachable!(),
        }
    }
    ret = std::cmp::max(ret, one_s + last_one_s);

    if have_zero {
        ret
    } else {
        ret - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    }

    #[test]
    fn c() {
        assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    }
}

pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![1; nums.len()];
    for ptr in (1..=nums.len() - 1).rev() {
        ret[ptr - 1] = ret[ptr] * nums[ptr];
    }
    let mut l = 1;
    for ptr in 0..nums.len() {
        if ptr > 0 {
            l = l * nums[ptr - 1];
        }
        ret[ptr] = ret[ptr] * l;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}

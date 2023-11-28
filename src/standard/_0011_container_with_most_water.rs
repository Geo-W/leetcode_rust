pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ptr_left = 0;
    let mut ptr_right = height.len() - 1;
    let mut max = 0;
    while ptr_left < ptr_right {
        max = max.max(
            (ptr_right - ptr_left) as i32 * std::cmp::min(height[ptr_left], height[ptr_right]),
        );
        if height[ptr_left] > height[ptr_right] {
            ptr_right -= 1;
        } else {
            ptr_left += 1;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn b() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }

    #[test]
    fn c() {
        assert_eq!(
            max_area(vec![1, 8, 6, 6, 7, 8, 9, 2, 3, 4, 2, 5, 4, 8, 3, 7]),
            98
        );
    }
}

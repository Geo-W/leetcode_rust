pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut mid = 0;

    while low < high {
        mid = low + (high - low) / 2;
        if nums[mid] > nums[mid + 1] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn b() {
        assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }

    #[test]
    fn c() {
        assert_eq!(find_peak_element(vec![2, 1]), 0);
    }

    #[test]
    fn d() {
        assert_eq!(find_peak_element(vec![1, 2]), 1);
    }

    #[test]
    fn e() {
        assert_eq!(find_peak_element(vec![1, 5, 6, 7, 2, 3, 5, 0]), 3);
    }

    #[test]
    fn f() {
        assert_eq!(find_peak_element(vec![1]), 0);
    }
}

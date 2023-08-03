/// Given an array nums of size n, return the majority element.
/// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut cur = nums[0];
    let mut counter = 1;
    nums.iter().skip(1).for_each(|num| {
        if *num == cur {
            counter += 1;
        } else {
            counter -= 1;
            if counter <= 0 {
                cur = *num;
                counter = 1;
            }
        }
    });
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn a() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }


    #[test]
    fn b() {
        assert_eq!(majority_element(vec![3, 3, 4]), 3);
    }

    #[test]
    fn C() {
        assert_eq!(majority_element(vec![1, 3, 1, 1, 4, 1, 1, 5, 1, 1, 6, 2, 2]), 1);
    }
}

/// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
/// Note that you must do this in-place without making a copy of the array.
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut deviation = 0;
    for i in 0..nums.len(){
        if nums[i] != 0 {
            nums[i-deviation] = nums[i];
        } else {
            deviation +=1;
        }
    }
    for i in (nums.len() - deviation)..nums.len() {
        nums[i] = 0;
    }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut vec);
        assert_eq!(vec, vec![1,3,12,0,0]);
    }
}

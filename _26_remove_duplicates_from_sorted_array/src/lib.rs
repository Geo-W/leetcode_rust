pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //  let mut offset = 0;
    //  let length = nums.len();

    //  for i in 1..length{
    //     if nums[i-offset] == nums[i-1-offset]{
    //         nums.remove(i-offset);
    //         offset +=1;
    //     }
    //  }



    let mut i = 1;
    for j in 1..nums.len() {
        if nums[j] != nums[j - 1] {
            nums[i] = nums[j];
            i += 1;
        }
    }
    println!("{:?}", nums);
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    }

    #[test]
    fn a() {
        assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), 5)
    }
}

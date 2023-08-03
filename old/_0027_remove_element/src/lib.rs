// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     let mut count = 0;
//     // let mut num2 = nums.clone();
//     for i in nums.iter_mut(){
//         if i != &val{
//             nums[count] = *i;
//             count +=1;
//         }
//     }
//     count as i32
// }



pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;
    let len = nums.len();
    for i in 0..len{
        if nums[i] != val{
            nums[count] = nums[i];
            count +=1;
        }
    }

    count as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_element(&mut vec![3,2,2,3], 3), 2);
    }

    #[test]
    fn a(){
        assert_eq!(remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    }
}

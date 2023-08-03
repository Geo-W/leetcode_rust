pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut input_ptr = 0;
    let mut temp = 999999;
    let mut token = 0;
    for i in 0..nums.len() {
        match token {
            0 | 1 => {
                if nums[i] == temp {
                    token += 1;
                } else {
                    temp = nums[i];
                    println!("temp : {:?}", temp);
                    token = 1;
                }
                nums[input_ptr] = nums[i];
                input_ptr += 1;
            }
            2 => {
                if nums[i] == temp {} else {
                    nums[input_ptr] = nums[i];
                    input_ptr += 1;
                    temp = nums[i];
                    token = 1;
                }
            }
            _ => {}
        }
    }
    input_ptr as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    }

    #[test]
    fn a() {
        assert_eq!(remove_duplicates(&mut vec![1,1,1,2,2,3]), 5);
    }

    #[test]
    fn b() {
        assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,1,2,3,3]), 7);
    }

    #[test]
    fn c() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2, 2, 2, 2]), 4);
    }
}

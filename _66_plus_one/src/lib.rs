pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let len = digits.len() - 1;
    let mut i = len.clone();
    // if digits[i] < 9 {
    //     digits[i] +=1;
    //     return digits
    // } else {
    loop {
        match digits[i] {
            9 | 10 => {
                digits[i] = 0;
                match i {
                    0 => {
                        digits.insert(0, 1);
                        break;
                    }
                    _ => {
                        match digits[i - 1] {
                            0..=8 => {
                                digits[i - 1] += 1;
                                i -= 1;
                                break;
                            }
                            _ => {
                                i -= 1;
                            }
                        }
                    }
                }
            }
            _ if i == len => {
                digits[i] += 1;
                break;
            }
            _ => {
                break;
            }
        }
    }
    // }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn a() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn b() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn c() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}

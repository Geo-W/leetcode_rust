use std::collections::HashSet;
use std::num::Wrapping;

pub fn my_atoi(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut status = false;
    let mut result_multiplier: i32 = 1;
    for i in s.chars() {
        match status {
            true => {
                match i {
                    '0'..='9' => add_result(i, result_multiplier, &mut result),
                    _ => break
                }
            }
            false => {
                match i {
                    '-' => {
                        result_multiplier = -1;
                        status = true
                    }
                    '+' => status = true,
                    '0'..='9' => {
                        add_result(i, result_multiplier, &mut result);
                        status = true
                    }
                    ' ' => continue,
                    _ => break
                }
            }
        }
    }
    return result;
}

pub fn add_result(i:char, result_multiplier:i32, result:&mut i32) {
    match result_multiplier{
        1 => *result = result.saturating_mul(10).saturating_add(i.to_digit(10).unwrap() as i32),
        -1 => *result = result.saturating_mul(10).saturating_sub(i.to_digit(10).unwrap() as i32),
        _ => panic!("only 1 and -1 is allowed")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = my_atoi("----55".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn a() {
        let result = my_atoi("-91283472332".to_string());
        assert_eq!(result, -2147483648)
    }

    #[test]
    fn b() {
        let result = my_atoi("-".to_string());
        assert_eq!(result, 0)
    }

    #[test]
    fn c() {
        let result = my_atoi("+1".to_string());
        assert_eq!(result, 1)
    }

    #[test]
    fn d() {
        let result = my_atoi("+-12".to_string());
        assert_eq!(result, 0)
    }

    #[test]
    fn e() {
        let result = my_atoi("1234567890123456789012345678901234567890".to_string());
        assert_eq!(result, 2147483647)
    }

    #[test]
    fn f() {
        let result = my_atoi("  0000000000012345678".to_string());
        assert_eq!(result, 12345678)
    }

    #[test]
    fn g() {
        let result = my_atoi("41930 with words".to_string());
        assert_eq!(result, 41930)
    }
}
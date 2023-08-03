/// Given an integer, convert it to a roman numeral.
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut ret = String::new();
    while num > 0 {
        match num {
            1000.. => {
                for _ in 0..(num / 1000) {
                    ret.push_str("M");
                }
                num -= num / 1000 * 1000;
            }
            900.. => {
                ret.push_str("CM");
                num -= 900;
            }
            500.. => {
                ret.push_str("D");
                num -= 500;
            }
            400.. => {
                ret.push_str("CD");
                num -= 400;
            }
            100.. => {
                for _ in 0..(num / 100) {
                    ret.push_str("C");
                }
                num -= num / 100 * 100;
            }
            90.. => {
                ret.push_str("XC");
                num -= 90;
            }
            50.. => {
                ret.push_str("L");
                num -= 50;
            }
            40.. => {
                ret.push_str("XL");
                num -= 40;
            }
            10.. => {
                for _ in 0..(num / 10) {
                    ret.push_str("X");
                }
                num -= num / 10 * 10;
            }
            9.. => {
                ret.push_str("IX");
                num -= 9;
            }
            5.. => {
                ret.push_str("V");
                num -= 5;
            }
            4.. => {
                ret.push_str("IV");
                num -= 4;
            }
            1.. => {
                for _ in 0..num {
                    ret.push_str("I");
                    num -= num;
                }
            }
            _ => {}
        }
    }
    ret
}


pub fn int_to_roman2(num: i32) -> String {
    use std::collections::HashMap;
    let map = HashMap::from([(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"),
        (4, "IV"), (1, "I")]);
    let arr = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut num = num;
    let mut ret = String::new();
    for i in 0..13 {
        if num > 0 {
            if num / arr[i] > 0 {
                let a = num / arr[i];
                for _ in 0..a {
                    ret.push_str(map.get(&arr[i]).unwrap());
                }
                num = num % arr[i];
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

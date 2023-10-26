/// Given an integer num, return the number of digits in num that divide num.
/// An integer val divides nums if nums % val == 0.
pub fn count_digits(num: i32) -> i32 {
    let mut new_num = num;
    let mut ret = 0;
    while new_num > 0 {
        if num % (new_num % 10) == 0 {
            ret += 1;
        }
        new_num /= 10;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(count_digits(7), 1);
    }

    #[test]
    fn b() {
        assert_eq!(count_digits(121), 2);
    }
    
    #[test]
    fn c() {
        assert_eq!(count_digits(1248), 4);
    }
}
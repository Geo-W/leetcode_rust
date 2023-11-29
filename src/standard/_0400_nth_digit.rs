pub fn find_nth_digit(mut n: i32) -> i32 {
    for i in 0..20 {
        if n / ((9 * 10_i32.pow(i as u32)) * (i + 1)) > 0 {
            n -= (9 * 10_i32.pow(i as u32)) * (i + 1);
        } else {
            let num2 = n / (i + 1) + 10_i32.pow(i as u32) - 1;
            let modulus = n % (i + 1);
            return if modulus != 0 {
                (num2 + 1) % 10_i32.pow((i - modulus + 2) as u32)
                    / 10_i32.pow((i - modulus + 1) as u32)
            } else {
                num2 % 10
            };
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_nth_digit(46567), 5);
    }

    #[test]
    fn b() {
        assert_eq!(find_nth_digit(2890), 1);
    }

    #[test]
    fn c() {
        assert_eq!(find_nth_digit(2889), 9);
    }

    #[test]
    fn d() {
        assert_eq!(find_nth_digit(23423235), 4);
    }

    #[test]
    fn e() {
        assert_eq!(find_nth_digit(11), 0);
    }
}

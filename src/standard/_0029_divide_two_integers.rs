pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }
    if divisor == 1 {
        return dividend;
    }
    let mut ret = 0;
    let non_neg = dividend < 0 && divisor < 0 || dividend > 0 && divisor > 0;
    if dividend > 0 {
        dividend = -dividend;
    }
    if divisor > 0 {
        divisor = -divisor;
    }

    while dividend <= divisor {
        let mut n = 0;
        for x in 0..31 {
            match divisor.checked_mul(1 << x) {
                None => {
                    break;
                }
                Some(v) => {
                    if v < dividend {
                        break;
                    }
                    n = x;
                }
            }
        }
        dividend -= divisor << n;
        ret += divisor * 2_i32.pow(n);
    }
    let multiplier = if non_neg { 1 } else { -1 };
    ret / (divisor) * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn b() {
        assert_eq!(divide(7, -3), -2);
    }

    #[test]
    fn c() {
        assert_eq!(divide(-7, 3), -2);
    }

    #[test]
    fn d() {
        assert_eq!(divide(1, 1), 1);
        assert_eq!(divide(0, 1), 0);
    }

    #[test]
    fn e() {
        for i in 1..25555 {
            assert_eq!(divide(i, i), 1);
        }

        for i in 2..25555 {
            assert_eq!(divide(i + 1, i), 1);
        }
    }

    #[test]
    fn f() {
        assert_eq!(divide(-2147483648, 2), -1073741824);
    }

    #[test]
    fn g() {
        assert_eq!(divide(-2147483648, -1), 2147483647);
    }

    #[test]
    fn h() {
        assert_eq!(divide(2147483647, 3), 715827882);
    }
}

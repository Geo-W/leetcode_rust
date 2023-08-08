/// Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
/// Return the maximum product you can get.
pub fn integer_break(n: i32) -> i32 {
    match n {
        1..=3 => n - 1,
        _ => {
            let mut coef = n / 3;
            let mut remain = n % 3;
            if remain == 1 {
                coef -= 1;
                remain += 3;
            } else if remain == 0 {
                remain = 1;
            }
            3_i32.pow(coef as u32) * remain
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::standard::_0343_integer_break::integer_break;

    #[test]
    fn t1() {
        assert_eq!(integer_break(3),2);
    }

    #[test]
    fn t2() {
        assert_eq!(integer_break(28),26244);

    }
}
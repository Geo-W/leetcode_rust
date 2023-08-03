pub fn reverse(mut x: i32) -> i32 {
    let mut remainder = 0;

    let mut result: i32 = 0;
    let mut multiplier = 10;


    while x != 0 {
        remainder = x % 10;
        // result = result.saturating_mul(multiplier).saturating_add(remainder);
        match result.checked_mul(multiplier) {
            Some(value) => result = value,
            None => {
                result = 0;
                break;
            }
        }

        match result.checked_add(remainder) {
            Some(value) => result = value,
            None => {
                result = 0;
                break;
            }
        }
        x = x / 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = reverse(123);
        assert_eq!(result, 321);
    }

    #[test]
    fn b() {
        let result = reverse(-123);
        assert_eq!(result, -321);
    }

    #[test]
    fn c() {
        let result = reverse(120);
        assert_eq!(result, 21);
    }

    #[test]
    fn d() {
        let result = reverse(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn e() {
        let result = reverse(123);
        assert_eq!(result, 321);
    }

    #[test]
    fn f() {
        let result = reverse(15647952);
        assert_eq!(result, 25974651);
    }

    #[test]
    fn g() {
        let result = reverse(1534236469);
        assert_eq!(result, 0);
    }
}

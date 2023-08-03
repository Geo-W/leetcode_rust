/// Given a positive integer n, find the pivot integer x such that:
///     The sum of all elements between 1 and x inclusively equals the sum of all elements between x and n inclusively.
/// Return the pivot integer x. If no such integer exists, return -1. It is guaranteed that there will be at most one pivot index for the given input.
pub fn pivot_integer(n: i32) -> i32 {
    let n: f64 = ((n as f64 * n as f64 + n as f64) / 2.0);
    if n.sqrt() % 1.0 == 0.0 {
        n.sqrt() as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(pivot_integer(8), 6);
    }

    #[test]
    fn a() {
        assert_eq!(pivot_integer(1), 1);
    }

    #[test]
    fn b() {
        assert_eq!(pivot_integer(4), -1);
    }
}

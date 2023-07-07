pub fn fib(n: i32) -> i32 {
    if n > 1 {
        fib(n - 2) + fib(n - 1)
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(4), 3);
    }
}

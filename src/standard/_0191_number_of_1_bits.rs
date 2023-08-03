/// Write a function that takes the binary representation of an unsigned integer and returns the number of '1' bits it has (also known as the Hamming weight).a
pub fn hammingWeight(n: u32) -> i32 {
    let mut counter = 0;
    let mut n = n;
    for _ in 1..=32 {
        if n % 2 != 0 {
            counter += 1;
        }
        n = n >> 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hammingWeight(0b00000000000000000000000000001011);
        assert_eq!(result, 3);
    }

    #[test]
    fn a() {
        let result = hammingWeight(00000000000000000000000010000000);
        assert_eq!(result, 1);
    }


    #[test]
    fn b() {
        let result = hammingWeight(11111111111111111111111111111101);
        assert_eq!(result, 31);
    }
}

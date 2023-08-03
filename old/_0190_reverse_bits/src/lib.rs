/// Reverse bits of a given 32 bits unsigned integer.
/// sample: in 43261596 (00000010100101000001111010011100)
/// out 964176192 (00111001011110000010100101000000)
pub fn reverse_bits(x: u32) -> u32 {
    let mut result = 0;
    let mut x = x;
    for _ in 1..=32 {
        // println!("{:?}",x);
        if x % 2 != 0 {
            result = 2 * result + 1;
        } else {
            result = 2 * result;
        }
        x = x >> 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse_bits(43261596), 964176192 );
    }

    #[test]
    fn a() {
        assert_eq!(reverse_bits(4294967293), 3221225471 )
    }
}

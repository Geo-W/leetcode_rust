/// Write a function that reverses a string. The input string is given as an array of characters s.
/// You must do this by modifying the input array in-place with O(1) extra memory.
pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len / 2) {
        s.swap(i, len - 1 - i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut vec);
        assert_eq!(vec, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn a() {
        let mut vec = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut vec);
        assert_eq!(vec, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}

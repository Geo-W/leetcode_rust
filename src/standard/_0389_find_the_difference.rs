pub fn find_the_difference(s: String, t: String) -> char {
    let a = s.as_bytes().iter().map(|x| *x as u32).sum::<u32>();
    let b = t.as_bytes().iter().map(|x| *x as u32).sum::<u32>();
    unsafe { std::char::from_u32_unchecked(b - a) }
}


#[cfg(test)]
mod tests {
    use crate::standard::_0389_find_the_difference::find_the_difference;

    #[test]
    fn it_works() {
        assert_eq!(find_the_difference("asdf".to_string(), "asdfd".to_string()), 'd');
    }

    #[test]
    fn a() {
        assert_eq!(find_the_difference("".to_string(), "y".to_string()), 'y');
    }
    
    #[test]
    fn b() {
        assert_eq!(find_the_difference("abf".to_string(), "abfg".to_string()), 'g');
    }
}

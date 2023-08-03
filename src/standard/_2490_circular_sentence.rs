/// A sentence is circular if:
///     The last character of a word is equal to the first character of the next word.
///     The last character of the last word is equal to the first character of the first word.
pub fn is_circular_sentence(sentence: String) -> bool {
    let mut it = sentence.split(" ");
    let mut it2 = it.clone();
    if let Some(first) = it2.next() {
        loop {
            match (it.next(), it2.next()) {
                (Some(v1), Some(v2)) => {
                    println!("first");
                    println!("{:?},{:?}", v1, v2);
                    if v1.get((v1.len() - 1)..) != v2.get(0..=0) {
                        return false;
                    }
                }
                (Some(v1), None) => {
                    println!("second");
                    println!("{:?},{:?}", v1, 5);
                    if v1.get((v1.len() - 1)..) != first.get(0..=0) {
                        return false;
                    }
                }
                (_, _) => {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_circular_sentence("leetcode exercises sound delightful".to_string()), true);
    }

    #[test]
    fn a() {
        assert_eq!(is_circular_sentence("eetcode".to_string()), true);
    }

    #[test]
    fn b() {
        assert_eq!(is_circular_sentence("Leetcode is cool".to_string()), false);
    }
}

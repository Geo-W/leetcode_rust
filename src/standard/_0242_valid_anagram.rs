    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = [0; 26];
        for b  in s.as_bytes(){
            map[(b-97) as usize] +=1;
        }

        for b in t.as_bytes() {
            map[(b-97) as usize] -=1;
        }


        for i in map{
            if i != 0 {
                return false;
            }
        }

        true

    }


#[cfg(test)]
mod tests {
    use crate::standard::_0242_valid_anagram::is_anagram;

    #[test]
    fn it_works() {
        assert_eq!(is_anagram("anagramz".to_string(), "nagaramz".to_string()), true);
    }

    #[test]
    fn a() {
        assert_eq!(
            is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}

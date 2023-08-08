pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in s.chars() {
        match map.get_mut(&i) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(i, 1);
            }
        }
    }

    for (i,c) in s.chars().enumerate() {
        if *map.get(&c).unwrap() == 1{
            return i as i32;
        }
    }


    -1
}


#[cfg(test)]
mod tests {
    use crate::standard::_0387_first_unique_character_in_a_string::first_uniq_char;

    #[test]
    fn it_works() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0)
    }
}

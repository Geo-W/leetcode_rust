pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut vec = Vec::new();
    for x in strs.iter() {
        vec.push(x.as_bytes())
    }
    let length = vec[0].len();
    let mut result = Vec::new();
    'outer: for i in 0..length {
        let count = 0;
        let mut temp: &u8 = &0;
        for x in vec.iter() {
            match x.get(i) {
                Some(value) => {
                    if value == vec[0].get(i).unwrap_or(&0) {
                        temp = value;
                    } else {
                        break 'outer;
                    }
                }
                None => {
                    println!("result:{:?}", result);
                    break 'outer;
                }
            }
        }
        result.push(temp.clone());
    }

    return String::from_utf8(result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(longest_common_prefix(vec!["flower", "flow", "flight"].iter().map(|x| x.to_string()).collect()), "fl".to_string());
    }

    #[test]
    fn b() {
        assert_eq!(longest_common_prefix(vec!["dog","racecar","car"].iter().map(|x| x.to_string()).collect()), "".to_string());
    }

    #[test]
    fn c() {
        assert_eq!(longest_common_prefix(vec!["abc", "ad", "a"].iter().map(|x| x.to_string()).collect()), "a".to_string());
    }
}

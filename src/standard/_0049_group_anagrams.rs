/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
pub fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let map: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz".chars().enumerate().map(|(i, char)| (char, 2usize.pow((i) as u32))).collect();
    let mut result_map:HashMap<(usize, usize, u32), Vec<String>> = HashMap::new();

    for str in strs {
        let sum: usize = str.chars().map(|c| map.get(&c).unwrap()).sum();
        let length = str.chars().count();
        let char_sum:u32 = str.chars().map(|c| c.to_digit(36).unwrap_or(0)).sum();
        result_map.entry((sum, length, char_sum)).or_insert(vec![]).push(str);
    }
    result_map.values().cloned().collect()
} // memory 40.43%, time 88.3%

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let map: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz".chars().enumerate().map(|(i, char)| (char, 2usize.pow((i) as u32))).collect();
    println!("{:?}", map);
    let mut result: Vec<Vec<String>> = vec![];
    let mut result_sum: Vec<(usize, usize, u32)> = vec![];
    for str in strs {
        let sum: usize = str.chars().map(|c| map.get(&c).unwrap()).sum();
        let length = str.chars().count();
        let char_sum:u32 = str.chars().map(|c| c.to_digit(36).unwrap_or(0)).sum();
        let (check_exist_result, index) = check_existing(&mut result_sum, sum, length, char_sum);
        if !check_exist_result {
            result_sum.push((sum, length, char_sum));
            result.push(vec![str])
        } else {
            result[index.unwrap()].push(str)
        }
    }
    result
} // memory 97.87%, timing 5.32%

pub fn check_existing(vec: &mut Vec<(usize, usize, u32)>, checker: usize, length: usize, char_sum: u32) -> (bool, Option<usize>) {
    let mut result = false;
    let mut index: Option<usize> = None;
    for (i, v) in vec.iter().enumerate() {
        if v.0 ^ checker == 0 && v.1 == length && v.2 == char_sum {
            result = true;
            index = Some(i);
            break;
        }
    };
    (result, index)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        group_anagrams(vec!["aabc", "tea", "tan", "ate", "nat", "bat", "bbc"].iter().map(|x| x.to_string()).collect());
        assert_eq!(4, 4);
    }

    #[test]
    fn a() {
        let a = group_anagrams(vec!["ron", "huh", "gay", "tow", "moe", "tie", "who", "ion", "rep", "bob",
                            "gte", "lee", "jay", "may", "wyo", "bay", "woe", "lip", "tit", "apt", "doe",
                            "hot", "dis", "fop", "low", "bop", "apt", "dun", "ben", "paw", "ere", "bad",
                            "ill", "fla", "mop", "tut", "sol", "peg", "pop", "les"].iter().map(|x| x.to_string()).collect());
        assert_eq!(4,4);
    }
}



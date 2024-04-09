pub fn max_product(words: Vec<String>) -> i32 {
    let mut ret = 0 ;
    let vec = words.iter().map(|s| s.bytes().fold(0i32, |mut acc, nxt| {
        acc |= 2i32.pow(nxt as u32 - 97);
        return acc;
    })).collect::<Vec<i32>>();
    for x in 0..vec.len()-1 {
        for y in (x+1)..vec.len() {
            if vec[x] ^ vec[y] == vec[x] + vec[y] {
                ret = std::cmp::max(ret, words[x].len()*words[y].len())
            }
        }
    }

    ret as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            max_product(vec_string!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
            16
        );
    }
    
    #[test]
    fn b() {
        assert_eq!(max_product(vec_string!["a","aa","aaa","aaaa"]), 0);
    }
}

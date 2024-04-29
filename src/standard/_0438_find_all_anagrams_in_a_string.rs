pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let len = p.len();
    let target = p.bytes().map(|x| x - 97).fold([0; 26], |mut acc, x| {
        acc[x as usize] += 1;
        return acc;
    });
    let mut ret = vec![];
    let mut window = [0; 26];
    let s_iter = s.bytes().map(|x| x as usize - 97).collect::<Vec<usize>>();

    for i in 0..len {
        window[s_iter[i]] += 1;
    }
    if window == target {
        ret.push(0);
    }

    for (idx, _) in s_iter.iter().enumerate().take(s_iter.len() - len) {
        window[s_iter[idx]] -= 1;
        window[s_iter[idx + len]] += 1;
        if window == target {
            ret.push(idx as i32 + 1);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            find_anagrams("aaaaaaaaaa".to_string(), "aaaaaaaaaaaaa".to_string()),
            vec![]
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            find_anagrams("aaaaaaaaaaaaa".to_string(), "aaaaaaaaaaaaa".to_string()),
            vec![0]
        );
    }
}

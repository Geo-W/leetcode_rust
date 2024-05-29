pub fn maximum_length(s: String) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut last_char = ' ';
    let mut cur_string = String::new();

    let chars = s.chars().collect::<Vec<_>>();

    for idx in 0..chars.len() {
        cur_string.clear();
        last_char = ' ';
        'inner_loop: for inner_idx in idx..chars.len() {
            let char = chars[inner_idx];
            if char != last_char && last_char != ' ' {
                cur_string.clear();
                break 'inner_loop;
            }
            cur_string.push(char);
            last_char = char;
            *map.entry(cur_string.clone()).or_insert(0) += 1;
        }
    }
    match map
        .into_iter()
        .filter(|x| x.1 >= 3)
        .max_by_key(|x| x.0.len())
    {
        None => -1,
        Some(x) => x.0.len() as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(maximum_length("aaaa".to_string()), 2);
    }

    #[test]
    fn b() {
        assert_eq!(maximum_length("abcef".to_string()), -1);
    }

    #[test]
    fn c() {
        assert_eq!(maximum_length("abcaba".to_string()), 1);
    }
}

pub fn compress(chars: &mut Vec<char>) -> i32 {
    fn edit(chars: &mut Vec<char>, cnt: usize, idx: usize, dev: usize) {
        let str = cnt.to_string();
        let mut char_replace_start_dev = 2;
        for num_char in str.chars() {
            chars[idx + char_replace_start_dev - dev] = num_char;
            char_replace_start_dev += 1;
        }
        for remove_idx in ((idx + char_replace_start_dev - dev)..=(idx + cnt - dev)).rev() {
            chars.remove(remove_idx);
        }
    }

    let mut last_char = ' ';
    let mut cnt = 1;
    for idx in (0..chars.len()).rev() {
        if chars[idx] == last_char {
            cnt += 1;
        } else {
            if cnt > 1 {
                edit(chars, cnt, idx, 0);
            }
            cnt = 1;
            last_char = chars[idx];
        }
    }
    if cnt > 1 {
        edit(chars, cnt, 0, 1);
    }

    chars.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut ret = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(&mut ret), 6);
        assert_eq!(ret, vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn b() {
        let mut ret = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(compress(&mut ret), 4);
        assert_eq!(ret, vec!['a', 'b', '1', '2']);
    }
}

pub fn maximum_binary_string(mut binary: String) -> String {
    let s = unsafe { binary.as_bytes_mut() };
    let mut idx = 0;
    // record location of last changed 0, avoid repeating search.
    let mut last_found = None;

    while idx < s.len() - 1 {
        match s[idx] {
            b'0' => {
                if s[idx + 1] == b'0' {
                    s[idx] = b'1';
                    idx += 1;
                } else {
                    // find next zero, and change current s[idx] to 1 and s[idx+1] to 0, and s[idx_found_with_zero] to 1;
                    let res = find_next_zero(s, idx, last_found);
                    if !res.0 {
                        // cannot find another zero, return answer
                        break;
                    } else {
                        last_found = Some(res.1);
                    }
                }
            }
            _ => {
                idx += 1;
            }
        }
    }
    binary
}

fn find_next_zero(s: &mut [u8], cur: usize, last_found: Option<usize>) -> (bool, usize) {
    let mut found = false;
    let mut start_point = last_found.unwrap_or(cur) + 1;
    while start_point < s.len() {
        if s[start_point] == b'1' {
            start_point += 1;
        } else {
            found = true;
            break;
        }
    }
    if found {
        s[start_point] = b'1';
        s[cur + 1] = b'0';
        return (true, start_point);
    } else {
        (false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            maximum_binary_string("000110".to_string()),
            "111011".to_string()
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            maximum_binary_string("101010001010110111010101001010101".to_string()),
            "111111111111111101111111111111111".to_string()
        );
    }
}

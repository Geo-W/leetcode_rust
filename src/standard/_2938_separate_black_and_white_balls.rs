pub fn minimum_steps(s: String) -> i64 {
    let mut ret = 0;
    let mut last_zero = match s.rfind('0') {
        None => {
            return 0;
        }
        Some(v) => v,
    };
    for (idx, char) in s.chars().rev().enumerate().skip(s.len() - 1 - last_zero) {
        match char {
            '1' => {
                let cnt = idx - (s.len() - 1 - last_zero);
                last_zero -= 1;
                ret += cnt
            }
            '0' => {}
            _ => unreachable!(),
        }
    }
    ret as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_steps("101".to_string()), 1);
    }

    #[test]
    fn b() {
        assert_eq!(minimum_steps("010100000000111".to_string()), 17);
    }
}

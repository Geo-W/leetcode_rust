pub fn minimum_operations(num: String) -> i32 {
    let mut cnt_0 = 0;
    let mut cnt_5 = 0;
    let mut found_0 = false;
    let mut found_5 = false;
    let mut ret = vec![num.len() as i32];

    for char in num.chars().rev() {
        if !found_0 {
            if char != '0' {
                cnt_0 += 1;
            } else {
                found_0 = true;
            }
        } else {
            if char != '0' && char != '5' {
                cnt_0 += 1;
            } else {
                ret.push(cnt_0);
                break;
            }
        }
    }

    for char in num.chars().rev() {
        if !found_5 {
            if char != '5' {
                cnt_5 += 1;
            } else {
                found_5 = true;
            }
        } else {
            if char != '2' && char != '7' {
                cnt_5 += 1;
            } else {
                ret.push(cnt_5);
                break;
            }
        }
    }
    if found_0 {
        ret.push(num.len() as i32 - 1);
    }

    ret.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_operations("2245047".to_string()), 2);
    }

    #[test]
    fn b() {
        assert_eq!(minimum_operations("2908305".to_string()), 3);
    }
}

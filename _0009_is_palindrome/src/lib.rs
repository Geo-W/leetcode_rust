fn is_palindrome(x: i32) -> bool {
    let len = calculate_len(x);  //get len - 1
    let l = if len % 2 == 0 {
        len / 2
    } else {
        (len / 2) + 1
    };
    // println!("len :{len}");
    let mut temp1: i32 = 0;
    let mut temp2: i32 = 0;
    if x < 0 {
        return false;
    }
    if x == 0 {
        return true;
    }
    for i in 0..l {
        let a = (x - temp1) / 10_i32.pow((len - i) as u32);
        let b_temp = (x - temp2) % 10_i32.pow((i + 1) as u32);
        let b = b_temp / 10_i32.pow(i as u32);
        // println!("{a},{b_temp},{b}");
        temp1 += a * 10_i32.pow((len - i) as u32);
        temp2 += b_temp;
        if a != b {
            return false;
        }
    }
    return true;
}


fn calculate_len(x: i32) -> i32 {
    let mut result = 0;
    let mut i = x;
    while i >= 10 {
        i = i / 10;
        result += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn b() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn c() {
        assert_eq!(is_palindrome(10), false);
    }
}

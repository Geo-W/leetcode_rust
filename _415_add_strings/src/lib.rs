pub fn add_strings(num1: String, num2: String) -> String {
    let (mut n1, mut n2) = (num1.bytes().rev(), num2.bytes().rev());
    let mut carry = 0;
    let mut result = Vec::with_capacity(std::cmp::max(num1.len(), num2.len()));

    loop {
        match (n1.next(), n2.next()) {
            (None, None) => { break; }
            (i_option, j_option) => {
                let i = if let Some(v) = i_option { v - 48 } else { 0 };
                let j = if let Some(v) = j_option { v - 48 } else { 0 };
                let tmp = i + j + carry;
                carry = 0;
                if tmp >= 10 {
                    result.push(tmp - 10 + 48);
                    carry = 1
                } else {
                    result.push(tmp + 48);
                }
            }
        }
    }
    if carry != 0 {
        result.push(49);
    }
    std::str::from_utf8(result.into_iter().rev().collect::<Vec<u8>>().as_ref()).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_strings("11".to_string(), "123".to_string()), "134".to_string());
    }

    #[test]
    fn a() {
        assert_eq!(add_strings("456".to_string(), "77".to_string()), "533\
        ".to_string());
    }
}

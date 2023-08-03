/// Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
pub fn multiply(num1: String, num2: String) -> String {
    let (mut n1, mut n2) = (num1.bytes().rev(), num2.bytes().rev());
    let mut ret = Vec::with_capacity(std::cmp::max(num1.len(), num2.len()));
    if num1 =="0" || num2 == "0"{
        return "0".to_string()
    }
    for (enums, num2) in n2.enumerate() {
        let mut res = Vec::with_capacity(num1.len() +1);
        let mut carry = 0;
        for i in 0..enums {
            res.push(48);
        }
        for num1 in n1.clone() {
            let tmp = (num1 - 48) * (num2 - 48) + carry;
            carry = tmp / 10;
            let remain = tmp % 10;

            res.push(remain + 48);
        }
        if carry != 0 {
            res.push(carry + 48);
        }
        ret.push(res);
    }

    let a = ret.into_iter().reduce(|cur, next| {
        let mut result = Vec::new();
        let mut carry = 0;
        let (mut n1, mut n2) = (cur.iter(), next.iter());
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
        result
    }).unwrap();
    std::str::from_utf8(a.into_iter().rev().collect::<Vec<u8>>().as_ref()).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply("123".to_string(), "456".to_string());
        assert_eq!(result, "56088".to_string());
    }

    #[test]
    fn a() {
        let result = multiply("9".to_string(), "9".to_string());
        assert_eq!(result, "81".to_string());
    }

    #[test]
    fn b() {
        let result = multiply("999".to_string(), "202".to_string());
        assert_eq!(result, "201798".to_string());
    }

    #[test]
    fn c() {
        let result = multiply("9133".to_string(), "0".to_string());
        assert_eq!(result, "0".to_string());
    }

    #[test]
    fn d() {
        let result = multiply("0".to_string(), "9133".to_string());
        assert_eq!(result, "0".to_string());
    }
}
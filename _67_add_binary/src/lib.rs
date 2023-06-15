    pub fn add_binary(a: String, b: String) -> String {
            let (mut n1, mut n2) = (a.bytes().rev(), b.bytes().rev());
    let mut carry = 0;
    let mut result = Vec::new();


    loop {
        match (n1.next(), n2.next()) {
            (None, None) => { break; }
            (i_option, j_option) => {
                let i = if let Some(v) = i_option { v - 48 } else { 0 };
                let j = if let Some(v) = j_option { v - 48 } else { 0 };
                let tmp = i + j + carry;
                carry = 0;
                if tmp >= 2 {
                    result.push(tmp - 2 + 48);
                    carry = 1
                } else {
                    result.push(tmp + 48);
                }
            }
        }
    }
    if carry !=0{
        result.push(49);
    }
    std::str::from_utf8(result.into_iter().rev().collect::<Vec<u8>>().as_ref()).unwrap().to_string()
    }

#[cfg(test)]
mod tests {
    use super::*;

}

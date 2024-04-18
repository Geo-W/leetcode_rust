pub fn final_string(s: String) -> String {
    let mut ret = Vec::new();

    for i in s .chars(){
        match i {
            'i' => ret.reverse(),
            _ => {
                ret.push(i);
            }
        }
    }

    String::from_iter(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(final_string("string".to_string()), "rtsng".to_string());
    }
}

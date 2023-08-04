/// count_and_say(1) = "1"
/// count_and_say(n) = saying digits from count_and_say(n-1)
/// e.g. "3322251" => "two 3 three 2 one 5 one 1" => "23321511"
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    } else {
        let mut ret = String::new();
        let mut tmp_char = 'a';
        let mut count = 0;
        for i in count_and_say(n - 1).chars() {
            if tmp_char == i {
                count += 1;
            } else {
                ret.push_str(&count.to_string());
                ret.push(tmp_char);
                tmp_char = i;
                count = 1;
            }
        }
        ret.push_str(&count.to_string());
        ret.push(tmp_char);
        ret[2..].to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::standard::_0038_count_and_say::count_and_say;

    #[test]
    fn t1() {
        assert_eq!(
            count_and_say(4),
            "1211".to_string()
        )
    }

    #[test]
    fn t2() {
        assert_eq!(
            count_and_say(9),
            "31131211131221".to_string()
        )
    }
}
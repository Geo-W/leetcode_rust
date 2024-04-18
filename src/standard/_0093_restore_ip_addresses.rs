pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut ret = vec![];
    let mut cur = String::new();

    let s = s.chars().collect::<Vec<char>>();

    fn dfs(ret: &mut Vec<String>, cur: &mut (usize, String), s: &Vec<char>, ptr: usize) {
        if cur.0 > 4 {
            return;
        }
        let mut tmp = vec![];
        let mut tmp_str = String::new();
        for (length, idx) in (ptr..ptr + 3).enumerate() {
            if idx >= s.len() {
                break;
            }

            tmp_str.push(s[idx]);
            let tmp_str_i32 = tmp_str.parse::<i32>().unwrap();
            if tmp_str_i32 == 0 {
                tmp.push((length, tmp_str.clone()));
                break;
            } else if tmp_str_i32 <= 255 {
                tmp.push((length, tmp_str.clone()));
            } else {
                tmp_str.pop();
                break;
            }
        }

        for (idx, str) in tmp {
            cur.1.push_str(&str);
            cur.1.push('.');
            cur.0 += 1;
            if cur.1.chars().count() == s.len() + 4 && cur.0 == 4 {
                cur.1.pop();
                ret.push(cur.1.clone());
                cur.1.push('.');
            } else {
                dfs(ret, cur, s, ptr + idx + 1);
            }
            for _ in 0..str.chars().count() + 1 {
                cur.1.pop();
            }
            cur.0 -= 1;
        }
    }

    dfs(&mut ret, &mut (0, cur), &s, 0);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            restore_ip_addresses("25525511135".to_string()),
            vec_string!["255.255.11.135", "255.255.111.35"]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            restore_ip_addresses("101023".to_string()),
            vec_string![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}

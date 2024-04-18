use std::collections::HashMap;

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    for str in cpdomains {
        let m = str.split(' ').collect::<Vec<&str>>();
        let n = m[1].split('.').collect::<Vec<&str>>();
        let c = n.iter().rev().fold("".to_string(), |acc, x| {
            let tmp = format!("{}.{}", x, acc);
            *map.entry(tmp.clone()).or_insert(0) += m[0].parse::<i32>().unwrap();
            tmp
        });
    }

    map.into_iter()
        .map(|(mut str, cnt)| {
            str.pop();
            format!("{} {}", cnt, str)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            subdomain_visits(vec_string!["9001 discuss.leetcode.com"]),
            vec_string!["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"]
        );
    }
}

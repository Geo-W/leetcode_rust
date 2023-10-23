pub fn count_seniors(details: Vec<String>) -> i32 {
    details.iter().fold(0, |cur, nxt| {
        if nxt.get(11..=12).unwrap().parse::<i32>().unwrap() > 60 {
            cur + 1
        } else {
            cur
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        let details = vec_string!["7868190130M7522", "5303914400F9211", "9273338290F4010"];
        assert_eq!(count_seniors(details), 2);
    }
}

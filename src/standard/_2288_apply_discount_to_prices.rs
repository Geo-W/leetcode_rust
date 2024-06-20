pub fn discount_prices(sentence: String, discount: i32) -> String {
    let mut ret = vec![];
    for word in sentence.split(' ') {
        if let Ok(v) = &word[1..word.len()].parse::<u64>() {
            if &word[0..=0] == "$" {
                ret.push(format!(
                    "${:.2}",
                    (*v as f64) - (*v as f64) * discount as f64 / 100.00
                ));
            } else {
                ret.push(word.to_string());
            }
        } else {
            ret.push(word.to_string());
        }
    }
    ret.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            discount_prices("there are $1 $2 and 5$ candies in the shop".to_string(), 50),
            "there are $0.50 $1.00 and 5$ candies in the shop"
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            discount_prices(
                "706hzu76jjh7yufr5x9ot60v149k5 $7651913186 pw2o $6".to_string(),
                28
            ),
            "706hzu76jjh7yufr5x9ot60v149k5 $5509377493.92 pw2o $4.32".to_string()
        );
    }
}

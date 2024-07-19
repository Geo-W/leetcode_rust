pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let mut power = power;
    power.sort();
    let mut power_s = vec![];
    let mut acc = 0;
    let mut last = i32::MAX;
    for p in power {
        if p != last {
            if acc > 0 {
                power_s.push((last as i64, acc));
            }
            last = p;
            acc = 1;
        } else {
            acc += 1;
        }
    }
    power_s.push((last as i64, acc));

    let mut dp = Vec::with_capacity(power_s.len());
    let mut ret = 0;

    for &(i, cnt) in &power_s {
        let tmp = match power_s.binary_search_by_key(&(i - 3), |x| x.0) {
            Ok(v) => dp[v] + i * cnt,
            Err(v) => {
                if v > 0 {
                    dp[v - 1] + i * cnt
                } else {
                    i * cnt
                }
            }
        };
        ret = std::cmp::max(ret, tmp);
        dp.push(ret);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(maximum_total_damage(vec![1, 1, 3, 4]), 6);
    }

    #[test]
    fn b() {
        assert_eq!(maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }

    #[test]
    fn c() {
        assert_eq!(
            maximum_total_damage(vec![5, 9, 2, 10, 2, 7, 10, 9, 3, 8]),
            31
        );
    }
}

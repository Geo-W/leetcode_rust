pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let sum_n = mean * (rolls.len() as i32 + n) - rolls.iter().sum::<i32>();
    match sum_n * 10 / n {
        10..=60 => {
            let mut ret = vec![sum_n / n; n as usize];
            for i in 0..sum_n % n {
                ret[i as usize] += 1;
            }
            ret
        }
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
    }

    #[test]
    fn b() {
        assert_eq!(missing_rolls(vec![3, 5, 3], 5, 3), Vec::<i32>::new());
    }
}

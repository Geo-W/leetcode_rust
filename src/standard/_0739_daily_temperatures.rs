pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![0; temperatures.len()];
    let mut mono = vec![];
    for (idx, item) in temperatures.iter().enumerate() {
        while let Some(v) = mono.last() {
            let v = *v;
            if *item > temperatures[v] {
                ret[v] = (idx - v) as i32;
                mono.pop();
            } else {
                break;
            }
        }
        mono.push(idx);
    }
    ret.truncate(temperatures.len());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn b() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    }

    #[test]
    fn c() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}

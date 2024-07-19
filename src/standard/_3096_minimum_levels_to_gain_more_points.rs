pub fn minimum_levels(possible: Vec<i32>) -> i32 {
    let mut ret = 1;
    let possible = possible
        .into_iter()
        .map(|x| if x == 0 { -1 } else { x })
        .collect::<Vec<_>>();
    let mut a = possible[0];
    let mut b = possible.iter().skip(1).sum::<i32>();
    while a <= b && ret < possible.len() - 1 {
        a += possible[ret];
        b -= possible[ret];
        ret += 1;
    }

    if a <= b {
        return -1;
    }
    ret as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_levels(vec![1, 0, 1, 0]), 1);
    }

    #[test]
    fn ab() {
        assert_eq!(minimum_levels(vec![0, 1, 1]), -1);
    }
    #[test]
    fn c() {
        assert_eq!(minimum_levels(vec![0, 1, 0, 0]), 2);
    }
}

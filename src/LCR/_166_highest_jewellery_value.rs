pub fn jewellery_value(mut frame: Vec<Vec<i32>>) -> i32 {
    for y in 0..frame.len() {
        for x in 0..frame[0].len() {
            let mut s1 = 0;
            let mut s2 = 0;
            if x > 0 {
                s1 = frame[y][x - 1];
            }
            if y > 0 {
                s2 = frame[y - 1][x];
            }
            frame[y][x] = std::cmp::max(s1, s2) + frame[y][x];
        }
    }

    *frame.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            jewellery_value(vec_vec![[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
            12
        );
    }
}

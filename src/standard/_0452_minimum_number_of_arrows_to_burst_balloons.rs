use std::cmp::min;

pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_by_key(|x| x[0]);
    let mut ret = 0;
    for ptr in 1..(points.len()) {
        if points[ptr][0] <= points[ptr - 1][1] {
            points[ptr][1] = min(points[ptr][1], points[ptr - 1][1]);
            ret += 1;
        }
    }
    (points.len() - ret) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_min_arrow_shots(vec_vec![[10, 16], [2, 8], [1, 6], [7, 12]]),
            2
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            find_min_arrow_shots(vec_vec![[1, 2], [2, 3], [3, 4], [4, 5]]),
            2
        );
    }

    #[test]
    fn c() {
        assert_eq!(find_min_arrow_shots(vec_vec![[1, 2]]), 1);
    }

    #[test]
    fn d() {
        assert_eq!(find_min_arrow_shots(vec_vec![[2, 3], [2, 3]]), 1);
    }
}

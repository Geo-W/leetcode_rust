pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        (p1[0] - p2[0]).abs().pow(2) + (p1[1] - p2[1]).abs().pow(2)
    }

    let (left_2, right_1, right2) = if p1[0] == p2[0] {
        (p2, p3, p4)
    } else if p1[0] == p3[0] {
        (p3, p2, p4)
    } else {
        (p4, p2, p3)
    };
    let left_1 = p1;

    let d_left = distance(&left_1, &left_2);
    let d_right = distance(&right_1, &right2);
    let d_1_1 = distance(&left_1, &right_1);
    let d_1_2 = distance(&left_1, &right2);
    let d_2_2 = distance(&left_2, &right2);

    d_left == d_right
        && d_1_1 == d_2_2
        && (d_1_2 == d_left || d_1_1 == d_left || d_1_1 == d_1_2)
        && d_left != 0
        && d_1_1 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        // distance(vec![1, 0], vec![0, 1]);
    }
    #[test]
    fn a() {
        assert_eq!(
            valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            valid_square(vec![1, 0], vec![0, 1], vec![0, -1], vec![-1, 0]),
            true
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            valid_square(vec![0, 0], vec![5, 0], vec![5, 4], vec![0, 4]),
            false
        );
    }
}

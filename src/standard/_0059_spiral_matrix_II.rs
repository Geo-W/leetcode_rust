use crate::standard::_0054_spiral_matrix::Direction;

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut left = 0;
    let mut right = n as usize - 1;
    let mut top = 0;
    let mut bottom = n as usize - 1;

    let mut ret = vec![vec![0; n as usize]; n as usize];
    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::GoRight;
    let mut start = 1;
    ret[0][0] = start;
    loop {
        if direction.margin_check(x, y, &mut left, &mut right, &mut top, &mut bottom) {
            direction.shrink(&mut left, &mut right, &mut top, &mut bottom);
            direction.turn_side();
            if direction.margin_check(x, y, &mut left, &mut right, &mut top, &mut bottom) {
                break;
            }
        }
        direction.go(&mut x, &mut y);
        start += 1;
        ret[y][x] = start;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            generate_matrix(3),
            vec_vec![[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(generate_matrix(1), vec_vec![[1]]);
    }
}

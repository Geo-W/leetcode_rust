pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left = 0;
    let mut right = matrix[0].len() - 1;
    let mut top = 0;
    let mut bottom = matrix.len() - 1;

    let mut ret = vec![];
    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::GoRight;
    ret.push(matrix[0][0]);
    loop {
        if direction.margin_check(x, y, &mut left, &mut right, &mut top, &mut bottom) {
            direction.shrink(&mut left, &mut right, &mut top, &mut bottom);
            direction.turn_side();
            if direction.margin_check(x, y, &mut left, &mut right, &mut top, &mut bottom) {
                break;
            }
        }
        direction.go(&mut x, &mut y);
        ret.push(matrix[y][x]);
    }
    ret
}

#[derive(Copy, Clone)]
pub(crate) enum Direction {
    GoLeft,
    GoRight,
    GoUp,
    GoDown,
}

impl Direction {
    pub fn shrink(&self, left: &mut usize, right: &mut usize, top: &mut usize, bottom: &mut usize) {
        match self {
            Direction::GoLeft => {
                *bottom -= 1;
            }
            Direction::GoRight => {
                *top += 1;
            }
            Direction::GoUp => {
                *left += 1;
            }
            Direction::GoDown => {
                *right -= 1;
            }
        }
    }

    pub fn turn_side(&mut self) {
        *self = match self {
            Direction::GoLeft => Direction::GoUp,
            Direction::GoRight => Direction::GoDown,
            Direction::GoUp => Direction::GoRight,
            Direction::GoDown => Direction::GoLeft,
        }
    }

    pub fn go(&self, x: &mut usize, y: &mut usize) {
        match self {
            Direction::GoLeft => {
                *x -= 1;
            }
            Direction::GoRight => {
                *x += 1;
            }
            Direction::GoUp => {
                *y -= 1;
            }
            Direction::GoDown => {
                *y += 1;
            }
        }
    }

    /// Returns true if hits the margin
    pub fn margin_check(
        &self,
        x: usize,
        y: usize,
        left: &mut usize,
        right: &mut usize,
        top: &mut usize,
        bottom: &mut usize,
    ) -> bool {
        let (act, tar) = match self {
            Direction::GoLeft => (x, *left),
            Direction::GoRight => (x, *right),
            Direction::GoUp => (y, *top),
            Direction::GoDown => (y, *bottom),
        };
        match self {
            Direction::GoLeft | Direction::GoUp => {
                if act > tar {
                    false
                } else {
                    true
                }
            }
            Direction::GoRight | Direction::GoDown => {
                if act < tar {
                    false
                } else {
                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_vec;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            spiral_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            spiral_order(vec_vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn c() {
        assert_eq!(spiral_order(vec_vec![[1]]), vec![1]);
    }
}

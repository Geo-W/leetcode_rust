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
        if margin_check(
            x,
            y,
            &mut left,
            &mut right,
            &mut top,
            &mut bottom,
            direction,
        ) {
            shrink(direction, &mut left, &mut right, &mut top, &mut bottom);
            turn_side(&mut direction);
            /// check if cannot turn anymore
            if margin_check(
                x,
                y,
                &mut left,
                &mut right,
                &mut top,
                &mut bottom,
                direction,
            ) {
                break;
            }
        }
        go(&mut x, &mut y, &direction);
        ret.push(matrix[y][x]);
    }

    /// Returns true if hits the margin
    fn margin_check(
        x: usize,
        y: usize,
        left: &mut usize,
        right: &mut usize,
        top: &mut usize,
        bottom: &mut usize,
        direction: Direction,
    ) -> bool {
        let (act, tar) = match direction {
            Direction::GoLeft => (x, *left),
            Direction::GoRight => (x, *right),
            Direction::GoUp => (y, *top),
            Direction::GoDown => (y, *bottom),
        };
        match direction {
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

    fn go(x: &mut usize, y: &mut usize, direction: &Direction) {
        match direction {
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

    fn turn_side(direction: &mut Direction) {
        *direction = match direction {
            Direction::GoLeft => Direction::GoUp,
            Direction::GoRight => Direction::GoDown,
            Direction::GoUp => Direction::GoRight,
            Direction::GoDown => Direction::GoLeft,
        }
    }

    fn shrink(
        direction: Direction,
        left: &mut usize,
        right: &mut usize,
        top: &mut usize,
        bottom: &mut usize,
    ) {
        match direction {
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

    ret
}

#[derive(Copy, Clone)]
enum Direction {
    GoLeft,
    GoRight,
    GoUp,
    GoDown,
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

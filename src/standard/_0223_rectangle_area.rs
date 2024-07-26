pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    fn get_overlap_length(a1: i32, a2: i32, b1: i32, b2: i32) -> i32 {
        if a1 >= b1 && a1 <= b2 {
            return if a2 <= b2 { a2 - a1 } else { b2 - a1 };
        }
        if b1 >= a1 && b1 <= a2 {
            return if b2 <= a2 { b2 - b1 } else { a2 - b1 };
        }

        0
    }

    let x = get_overlap_length(ax1, ax2, bx1, bx2);
    let y = get_overlap_length(ay1, ay2, by1, by2);

    (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }
}

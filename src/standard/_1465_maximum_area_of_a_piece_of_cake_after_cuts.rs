/// Return the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays horizontalCuts and verticalCuts.
/// Since the answer can be a large number, return this modulo 109 + 7.
pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let a = split_area(horizontal_cuts, h);
    let b = split_area(vertical_cuts, w);
    ((a) as i64 * (b) as i64 % 1_000_000_007) as i32
}

fn split_area(cuts: Vec<i32>, max: i32) -> i32 {
    let mut cuts = cuts;
    cuts.sort();
    let mut ret = 0;
    let mut cur = 0;
    for cut in cuts {
        if cut - cur > ret {
            ret = cut - cur;
        }
        cur = cut;
    }
    if max - cur > ret {
        ret = max - cur;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_area(5, 4, vec![3, 1], vec![1]), 6);
    }

    #[test]
    fn b() {
        assert_eq!(max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
    }

    #[test]
    fn c() {
        assert_eq!(max_area(1000000000, 1000000000, vec![2], vec![2]), 81);
    }
}
pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for y in 1..triangle.len() {
        for x in 0..triangle[y].len() {
            let left = if x == 0 {
                i32::MAX
            } else {
                triangle[y-1][x-1]
            };
            let right = if x == triangle[y].len() - 1 {
                i32::MAX
            } else {
                triangle[y-1][x]
            };

            triangle[y][x] = std::cmp::min(left, right) + triangle[y][x];


        }
    }


    *triangle.last().unwrap().into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            minimum_total(vec_vec![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]),
            11
        );
    }
}

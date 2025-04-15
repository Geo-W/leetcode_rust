use std::collections::{HashMap, HashSet};

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    for idx in 0..points.len() {
        for idx2 in idx + 1..points.len() {
            let k = if points[idx2][0] - points[idx][0] == 0 {
                None
            } else {
                Some(
                    ((points[idx2][1] - points[idx][1]) as f32
                        / (points[idx2][0] - points[idx][0]) as f32
                        * 100000.00)
                        .round() as i32,
                )
            };

            let x_intersection = match k {
                None => {
                    Some(points[idx][0])
                }
                Some(0) => {
                    None
                }
                Some(k) => {
                    Some(
                        ((points[idx][0] as f32 - (points[idx][1] as f32 / k as f32)) * 1000.00).round()
                            as i32
                    )
                }
            };

            let y_intersection = match k {
                None => {
                    None
                }
                Some(0) => {
                    Some(points[idx][1])
                }
                Some(k) => {
                    Some(points[idx][1] * 100000 + k * points[idx][0])
                }
            };

            let s = map.entry((x_intersection, y_intersection, k)).or_insert_with(HashSet::new);
            s.insert((points[idx][0], points[idx][1]));
            s.insert((points[idx2][0], points[idx2][1]));
        }
    }

    map.into_iter()
        .max_by_key(|x| x.1.len())
        .map(|x| x.1.len() as i32)
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(max_points(vec_vec![[1, 1], [2, 2], [3, 3]]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(
            max_points(vec_vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]),
            4
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            max_points(vec_vec![[3, 3], [1, 4], [1, 1], [2, 1], [2, 2]]),
            3
        );
    }

    #[test]
    fn d() {
        assert_eq!(
            max_points(vec_vec![
                [0, 0],
                [4, 5],
                [7, 8],
                [8, 9],
                [5, 6],
                [3, 4],
                [1, 1]
            ]),
            5
        );
    }

    #[test]
    fn e() {
        assert_eq!(max_points(vec_vec![[5151, 5150], [0, 0], [5152, 5151]]), 2);
    }

    #[test]
    fn g() {
        assert_eq!(
            max_points(vec_vec![
                [1, 1],
                [2, 10],
                [3, 100],
                [4, 1000],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [1, -1],
                [2, -10],
                [3, -100],
                [4, -1000]
            ]),
            4
        );
    }
}

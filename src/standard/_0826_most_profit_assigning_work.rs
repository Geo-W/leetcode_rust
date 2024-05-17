pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut dif_pro = difficulty
        .into_iter()
        .zip(profit.into_iter())
        .collect::<Vec<_>>();
    dif_pro.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let mut max = 0;
    for dp in dif_pro.iter_mut() {
        if dp.1 < max {
            dp.1 = max;
        } else {
            max = dp.1;
        }
    }
    worker
        .into_iter()
        .map(|x| match dif_pro.binary_search_by_key(&x, |d| d.0) {
            Ok(v) => dif_pro[v].1,
            Err(v) => {
                if v > 0 {
                    dif_pro[v - 1].1
                } else {
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]),
            0
        );
    }
    #[test]
    fn c() {
        assert_eq!(
            max_profit_assignment(
                vec![23, 30, 35, 35, 43, 46, 47, 81, 83, 98],
                vec![8, 11, 11, 20, 33, 37, 60, 72, 87, 95],
                vec![95, 46, 47, 97, 11, 35, 99, 56, 41, 92]
            ),
            553
        );
    }
}

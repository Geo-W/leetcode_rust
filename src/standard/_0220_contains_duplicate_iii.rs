pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
    let mut arr = nums
        .into_iter()
        .enumerate()
        .map(|x| (x.1, x.0))
        .collect::<Vec<_>>();
    arr.sort();
    let mut ptr1 = 0;
    let mut ptr2 = 1;
    let mut arr2: Vec<(i32, Vec<i32>)> = Vec::new();
    for num in arr {
        if let Some((v, a)) = arr2.last_mut() {
            if *v == num.0 {
                a.push(num.1 as i32);
            } else {
                arr2.push((num.0, vec![num.1 as i32]));
            }
        } else {
            arr2.push((num.0, vec![num.1 as i32]));
        }
    }
    while ptr2 <= arr2.len() - 1 && ptr1 != ptr2 {
        if (arr2[ptr2].0 - arr2[ptr1].0).abs() <= value_diff {
            for idx1 in arr2[ptr2].1.iter() {
                for idx2 in arr2[ptr1].1.iter() {
                    if (idx2 - idx1).abs() <= index_diff {
                        return true;
                    }
                }
            }
        }
        if ptr2 - ptr1 == 1 {
            ptr2 += 1;
        } else {
            ptr1 += 1;
        }
    }

    for (_, i) in arr2 {
        for idx in 0..(i.len() - 1) {
            if (i[idx + 1] - i[idx]).abs() <= index_diff {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![1, 2, 1, 1], 1, 0),
            true
        );
    }

    #[test]
    fn d() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![8, 7, 15, 1, 6, 1, 9, 15], 1, 3),
            true
        );
    }

    #[test]
    fn e() {
        assert_eq!(contains_nearby_almost_duplicate(vec![-2, 3], 2, 5), true);
    }

    #[test]
    fn f() {
        assert_eq!(contains_nearby_almost_duplicate(vec![1, 2], 1, 1), true);
    }

    #[test]
    fn g() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![0, 10, 22, 15, 0, 5, 22, 12, 1, 5], 3, 3),
            false
        );
    }

    #[test]
    fn h() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![10, 100, 11, 9, 100, 10], 1, 2),
            true
        );
    }

    #[test]
    fn i() {
        assert_eq!(contains_nearby_almost_duplicate(vec![0, 5, 0], 2, 4), true);
    }

    #[test]
    fn j() {
        assert_eq!(
            contains_nearby_almost_duplicate(vec![3, 6, 0, 4], 2, 2),
            true
        );
    }
}

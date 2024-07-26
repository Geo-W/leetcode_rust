pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let mut ret: Vec<(i32, i32, i32)> = vec![]; // (value, length, occurrence of such length)
    for num in nums {
        let idx = match ret.binary_search_by_key(&num, |x| x.0) {
            Ok(idx) => idx,
            Err(idx) => {
                ret.insert(idx, (num, 1, 0));
                idx
            }
        };
        let max_v = ret[0..idx].iter().map(|x| x.1).max(); // get max length for elements smaller than inserted num.
        match max_v {
            None => {
                ret[idx] = (ret[idx].0, ret[idx].1, ret[idx].2 + 1); // no elements are smaller, add 1 to the occurrence.
            }
            Some(v) => {
                let max_cnt = ret[0..idx]
                    .iter()
                    .filter(|x| x.1 == v)
                    .map(|x| x.2)
                    .sum::<i32>(); // get total occurrence count for elements smaller than inserted num with max length.
                if v + 1 > ret[idx].1 {
                    // if including current element, the length(v+1) is greater than the current one,
                    // replace the occurrence with max occurrence of previous elements.
                    ret[idx] = (ret[idx].0, v + 1, max_cnt);
                } else if v + 1 == ret[idx].1 {
                    // if including current element, the length is equal to the current one previously calculated,
                    // add the max occurrence cnt to it.
                    ret[idx] = (ret[idx].0, ret[idx].1, max_cnt + ret[idx].2);
                }
            }
        }
    }
    let max_s = ret.iter().map(|&(_, second, _)| second).max().unwrap();
    ret.into_iter()
        .filter(|&(_, second, _)| second == max_s)
        .map(|x| x.2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }

    #[test]
    fn b() {
        assert_eq!(find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }

    #[test]
    fn d() {
        assert_eq!(find_number_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn e() {
        assert_eq!(find_number_of_lis(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]), 27);
    }

    #[test]
    fn f() {
        assert_eq!(find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]), 3);
    }
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut ret: Vec<(i32, i32)> = vec![];
    for num in nums {
        let idx = match ret.binary_search_by_key(&num, |x| x.0) {
            Ok(idx) => idx,
            Err(idx) => {
                ret.insert(idx, (num, 1));
                idx
            }
        };
        let cnt = ret[0..idx].iter().max_by_key(|x| x.1).unwrap_or(&(0, 0)).1;
        ret[idx] = (ret[idx].0, std::cmp::max(ret[idx].1, cnt + 1));
    }

    ret.into_iter().map(|x| x.1).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn b() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn c() {
        assert_eq!(length_of_lis(vec![7, 7, 7]), 1);
    }
}

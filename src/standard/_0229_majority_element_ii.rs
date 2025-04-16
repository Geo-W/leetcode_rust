pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<(i32, usize)> = vec![];
    let len = nums.len();
    for num in &nums {
        let mut found = false;
        for (num_1, cnt) in ret.iter_mut() {
            if *num_1 == *num {
                *cnt += 1;
                found = true;
            }
        }

        if !found {
            if ret.len() < 2 {
                ret.push((*num, 1));
            } else {
                for idx in [1, 0] {
                    if ret[idx].1 == 1 {
                        ret.remove(idx);
                    } else {
                        ret[idx].1 -= 1;
                    }
                }
            }
        }
    }

    ret.iter_mut().for_each(|(_, x)| *x = 0);
    for num in nums {
        for (n, cnt) in ret.iter_mut() {
            if *n == num {
                *cnt += 1;
            }
        }
    }

    ret.into_iter()
        .filter(|x| x.1 > len / 3)
        .map(|x| x.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    }

    #[test]
    fn b() {
        assert_eq!(majority_element(vec![1]), vec![1]);
    }

    #[test]
    fn c() {
        assert_eq!(majority_element(vec![1, 2,]), vec![1, 2]);
    }

    #[test]
    fn d() {
        assert_eq!(majority_element(vec![2, 2, 1, 3]), vec![2]);
    }
}

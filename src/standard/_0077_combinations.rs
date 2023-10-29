/// Return all possible combinations of number from the range [1, n] with length k.
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    fn recur(tmp: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, cur: i32, n: i32, k: usize) {
        if tmp.len() == k {
            ret.push(tmp.clone());
        }

        for i in cur..=n {
            tmp.push(i);
            recur(tmp, ret, i+1, n, k);
            tmp.pop();
        }
    }

    recur(&mut tmp, &mut ret, 1, n, k as usize);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut rhs = vec![
            vec![2,4],
            vec![3,4],
            vec![2,3],
            vec![1,2],
            vec![1,3],
            vec![1,4]
        ];
        rhs.sort();
        let mut lhs = combine(4,2);
        lhs.sort();
        assert_eq!(lhs, rhs);
    }
}
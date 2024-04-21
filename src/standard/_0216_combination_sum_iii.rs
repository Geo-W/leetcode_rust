pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = vec![];
    fn dfs(
        ret: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
        tmp_sum: i32,
        checking_num: i32,
        target: i32,
        target_len: usize,
    ) {
        for i in checking_num..=9 {
            tmp.push(i);
            if tmp.len() == target_len && tmp_sum + i == target {
                ret.push(tmp.clone());
            } else if tmp.len() < target_len && tmp_sum + i < target {
                dfs(ret, tmp, tmp_sum + i, i + 1, target, target_len);
            }
            tmp.pop();
        }
    }

    dfs(&mut ret, &mut tmp, 0, 1, n, k as usize);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(combination_sum3(3, 7), vec_vec![[1, 2, 4]]);
    }

    #[test]
    fn b() {
        assert_eq!(
            combination_sum3(3, 9),
            vec_vec![[1, 2, 6], [1, 3, 5], [2, 3, 4]]
        );
    }
}

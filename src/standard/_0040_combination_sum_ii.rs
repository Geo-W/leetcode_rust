pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut cur = vec![];

    let mut vec = candidates
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect::<Vec<(i32, i32)>>();
    vec.sort_by_key(|x| x.0);

    fn dfs(
        candidate: &[(i32, i32)],
        target: i32,
        cur: &mut Vec<i32>,
        cur_sum: i32,
        ret: &mut Vec<Vec<i32>>,
    ) {
        for (idx, (item, count)) in candidate.iter().enumerate() {
            for multiplier in 0..*count {
                let multiplier = multiplier + 1;
                let tmp = cur_sum + (multiplier) * item;
                if tmp == target {
                    for _ in 0..multiplier {
                        cur.push(*item);
                    }
                    ret.push(cur.clone());
                    for _ in 0..multiplier {
                        cur.pop();
                    }
                } else if tmp < target {
                    for _ in 0..multiplier {
                        cur.push(*item);
                    }
                    dfs(
                        &candidate[(idx + 1)..candidate.len()],
                        target,
                        cur,
                        tmp,
                        ret,
                    );
                    for _ in 0..multiplier {
                        cur.pop();
                    }
                }
            }
        }
    }

    dfs(&vec, target, &mut cur, 0, &mut ret);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec_vec![[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]]
        );
    }
}

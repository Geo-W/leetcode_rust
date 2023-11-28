pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = Ans::default();

    fn recur(
        ret: &mut Vec<Vec<i32>>,
        tmp: &mut Ans,
        target: i32,
        candidates: &Vec<i32>,
        ptr: usize,
    ) {
        for idx in ptr..candidates.len() {
            let i = candidates[idx];
            if target - tmp.sum > i {
                tmp.add(i);
                recur(ret, tmp, target, candidates, idx);
                tmp.sub(i);
            } else if target - tmp.sum == i {
                tmp.add(i);
                ret.push(tmp.v.clone());
                tmp.sub(i);
            } else {
                continue;
            }
        }
    }

    recur(&mut ret, &mut tmp, target, &candidates, 0);

    ret
}

#[derive(Default)]
struct Ans {
    v: Vec<i32>,
    sum: i32,
}

impl Ans {
    fn add(&mut self, i: i32) {
        self.v.push(i);
        self.sum += i;
    }

    fn sub(&mut self, i: i32) {
        self.v.pop();
        self.sum -= i;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec_vec![[2, 2, 3], [7]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            combination_sum(vec![8, 7, 4, 3], 11),
            vec_vec![[8, 3], [7, 4], [4, 4, 3]]
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            combination_sum(vec![4, 2, 8], 8),
            vec_vec![[4, 4], [4, 2, 2], [2, 2, 2, 2], [8]]
        );
    }
}

use std::collections::HashSet;

pub fn gray_code(n: i32) -> Vec<i32> {
    let mut ret = vec![0];
    fn dfs(ret: &mut Vec<i32>, used: &mut HashSet<i32>, n: i32) -> bool {
        if ret.len() == 2_usize.pow(n as u32) {
            return true;
        }
        let mut last = *ret.last().unwrap();
        let first = *ret.first().unwrap();
        let mut all_false = true;
        for i in 0..n {
            let mut tmp = 1;
            tmp = tmp << i;
            let new = last ^ tmp;
            if !used.contains(&new) {
                if used.len() == 2_usize.pow(n as u32) - 1 && !check_contains(n, new ^ first) {
                    continue;
                }
                all_false = false;
                ret.push(new);
                used.insert(new);
                if dfs(ret, used, n) {
                    return true;
                }
                ret.pop();
                used.remove(&new);
            }
        }
        if all_false {
            return false;
        }
        return true;
    }
    dfs(&mut ret, &mut HashSet::from_iter([0]), n);
    ret
}

fn check_contains(n: i32, i: i32) -> bool {
    let mut ret = 1;
    for tmp in 0..n {
        if i == ret << tmp {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            gray_code(4),
            vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8]
        );
    }
}

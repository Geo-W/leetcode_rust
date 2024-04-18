use std::collections::HashMap;

pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
    let len = if changed.len() % 2 != 0 {
        return vec![];
    } else {
        changed.len() / 2
    };
    changed.sort_unstable();

    let mut cnt_map = changed.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0) += 1;
        return acc;
    });

    let mut deleted_cnt = HashMap::new();

    let mut checked_cnt = 0;
    let mut idx = 0;
    let mut ret = vec![];

    while checked_cnt < len {
        let item = changed[idx];
        idx += 1;
        if let Some(v) = deleted_cnt.get_mut(&item) {
            *v -= 1;
            if *v == 0{
                deleted_cnt.remove(&item);
            }
            continue;
        }
        for i in [item, item * 2] {
            match cnt_map.get_mut(&i) {
                None => return vec![],
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        cnt_map.remove(&i);
                    }
                }
            }
        }
        ret.push(item);
        checked_cnt += 1;
        *deleted_cnt.entry(item * 2).or_insert(0) += 1;
    }
    if cnt_map.is_empty() {
        return ret;
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_original_array(vec![1, 3, 4, 2, 6, 8]), vec![1, 3, 4]);
    }

    #[test]
    fn b() {
        assert_eq!(find_original_array(vec![6, 3, 0, 1]), vec![]);
    }

    #[test]
    fn c() {
        assert_eq!(find_original_array(vec![0, 0, 0, 0]), vec![0, 0]);
    }
}

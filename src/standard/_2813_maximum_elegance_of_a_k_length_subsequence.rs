use std::cmp::{max, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
    items.sort_by_key(|x| x[0]);
    items.reverse();

    let mut ret = 0;

    let mut map = HashMap::new();

    let mut duplicate_category = HashSet::new();

    let mut profit_now = 0;

    for idx in 0..k {
        let (profit, category) = (items[idx as usize][0], items[idx as usize][1]);
        profit_now += profit as i64;
        let entry = map.entry(category).or_insert_with(BinaryHeap::new);
        entry.push(Reverse(profit));
        if entry.len() > 1 {
            duplicate_category.insert(category);
        }
    }
    ret = max(ret, profit_now + map.len().pow(2) as i64);

    for idx in (k as usize)..items.len() {
        let (profit, category) = (items[idx as usize][0], items[idx as usize][1]);
        if !map.contains_key(&category) {
            //remove the category with duplicate and least price
            let (&min_category, min_profit) = match duplicate_category
                .iter()
                .map(|x| (x, *map.get(x).unwrap().peek().unwrap()))
                .min_by_key(|x| x.1 .0)
            {
                None => {
                    break;
                }
                Some(v) => (v.0, v.1),
            };
            profit_now -= min_profit.0 as i64;
            profit_now += profit as i64;

            let heap = map.get_mut(&min_category).unwrap();
            heap.pop().unwrap();
            if heap.len() <= 1 {
                duplicate_category.remove(&min_category);
            }
            map.entry(category)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(profit));
            ret = max(ret, profit_now + map.len().pow(2) as i64)
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_maximum_elegance(vec_vec![[3, 2], [5, 1], [10, 1]], 2),
            17
        );
    }

    #[test]
    fn b() {
        assert_eq!(find_maximum_elegance(vec_vec![[1, 2], [10, 1]], 1), 11);
    }
}

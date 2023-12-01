use std::collections::HashMap;

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    let mut x_list = vec![0; mat.len()];
    let mut y_list = vec![0; mat[0].len()];
    for (y, y_item) in mat.into_iter().enumerate() {
        for (x, x_item) in y_item.into_iter().enumerate() {
            map.insert(x_item, (x, y));
        }
    }

    for (idx, item) in arr.into_iter().enumerate() {
        let &(x, y) = map.get(&item).unwrap();
        x_list[y] += 1;
        if x_list[y] == y_list.len() {
            return idx as i32;
        }
        y_list[x] += 1;
        if y_list[x] == x_list.len() {
            return idx as i32;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            first_complete_index(vec![1, 3, 4, 2], vec_vec![[1, 4], [2, 3]]),
            2
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            first_complete_index(vec![1, 4, 5, 2, 6, 3], vec_vec![[4, 3, 5], [1, 2, 6]]),
            1
        );
    }
}

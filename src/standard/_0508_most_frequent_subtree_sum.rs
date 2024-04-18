use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut max = 0;

    fn dfs(map: &mut HashMap<i32, i32>, node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(v) = node {
            let n = v.borrow();
            let num = n.val + dfs(map, n.left.clone(), max) + dfs(map, n.right.clone(), max);

            let e = map.entry(num).or_insert(0);
            *e += 1;
            *max = std::cmp::max(*max, *e);
            return num;
        }
        0
    }

    dfs(&mut map, root, &mut max);

    map.into_iter()
        .filter(|x| x.1 == max)
        .map(|x| x.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(
            find_frequent_tree_sum(tree_from_vec![5, 2, -3]),
            vec![-3, 4, 2]
        );
    }
}

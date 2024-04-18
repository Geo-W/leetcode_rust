use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    if root.is_none() {
        return ret;
    }
    fn dfs(
        ret: &mut Vec<Vec<i32>>,
        target: i32,
        node: Rc<RefCell<TreeNode>>,
        cur: &mut Vec<i32>,
        cur_sum: i32,
    ) {
        let mut leaf_node = true;
        let node = node.borrow();
        cur.push(node.val);
        if let Some(v) = &node.left {
            leaf_node = false;
            dfs(ret, target, v.clone(), cur, cur_sum + node.val);
        }
        if let Some(v) = &node.right {
            leaf_node = false;
            dfs(ret, target, v.clone(), cur, cur_sum + node.val);
        }

        if leaf_node {
            if cur_sum + node.val == target {
                ret.push(cur.clone());
            }
        }
        cur.pop();
    }

    dfs(&mut ret, target_sum, root.unwrap(), &mut vec![], 0);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree_from_vec, vec_vec};

    #[test]
    fn a() {
        assert_eq!(
            path_sum(
                tree_from_vec![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1],
                22
            ),
            vec_vec![[5, 4, 11, 2], [5, 8, 4, 5]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(path_sum(tree_from_vec![1, 2, 3], 5), Vec::<Vec<i32>>::new());
    }
}

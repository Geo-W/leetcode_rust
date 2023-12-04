use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(last: &mut Option<i32>, node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(v) = node {
            let v = v.borrow();
            if !dfs(last, v.right.clone()) {
                return false;
            };
            if let Some(i) = last {
                if v.val >= *i {
                    return false;
                }
            }
            *last = Some(v.val);
            if !dfs(last, v.left.clone()) {
                return false;
            }
        }
        true
    }
    dfs(&mut None, root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(is_valid_bst(tree_from_vec![2, 1, 3]), true);
    }

    #[test]
    fn b() {
        assert_eq!(
            is_valid_bst(tree_from_vec![5, 1, 4, null, null, 3, 6]),
            false
        );
    }

    #[test]
    fn c() {
        assert_eq!(is_valid_bst(tree_from_vec![2147483647]), true);
    }
}

use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(sum: &mut i32, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(v) = node {
            let mut v = v.borrow_mut();
            dfs(sum, v.right.clone());
            *sum += v.val;
            v.val = *sum;
            dfs(sum, v.left.clone());
        }
    }
    let r = root.clone();
    dfs(&mut 0, r);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(
            bst_to_gst(tree_from_vec![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ]),
            tree_from_vec![30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]
        );
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use leetcode_utils::{TreeNode};

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn tra(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            let v = node.borrow();
            ret.push(v.val);
            tra(&v.left, ret);
            tra(&v.right, ret);
        }
    }
    let mut ret: Vec<i32> = vec![];
    tra(&root, &mut ret);
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tree = TreeNode::from_vec(vec![Some(1), None, Some(2), Some(3)]);
        let result = preorder_traversal(tree);
        assert_eq!(result, vec![1,2,3]);
    }
}

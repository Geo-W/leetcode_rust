use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::tree::TreeNode;

pub fn merge_trees(mut root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn tra(node: &mut Option<Rc<RefCell<TreeNode>>>, node2: &Option<Rc<RefCell<TreeNode>>>) {
        match (&node, &node2) {
            (Some(v1), Some(v2)) => {
                let mut v1 = v1.borrow_mut();
                let v2 = v2.borrow();
                v1.val = v1.val + v2.val;
                tra(&mut v1.left, &v2.left);
                tra(&mut v1.right, &v2.right);
            }
            (Some(v1), None) => {
                let mut v1 = v1.borrow_mut();
                tra(&mut v1.left, &None);
                tra(&mut v1.right, &None);
            }
            (None, Some(v2)) => {
                let v2 = v2.borrow();
                *node = Some(Rc::new(RefCell::new(TreeNode::new(v2.val))));
                let mut b = node.as_mut().unwrap().borrow_mut();
                tra(&mut b.left, &v2.left);
                tra(&mut b.right, &v2.right);
            }
            _ => {}
        }
    }
    tra(&mut root1, &root2);
    root1
}

#[cfg(test)]
mod tests {
    use crate::tree_from_vec;
    use super::*;

    #[test]
    fn it_works() {
        let tree = tree_from_vec![1,3,2,5];
        let tree2 = tree_from_vec![2,1,3,null,4,null,7];
        let result = merge_trees(tree, tree2);
        assert_eq!(result, tree_from_vec![3,4,5,5,4,null,7]);
    }
}

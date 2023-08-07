use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::TreeNode;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn tra(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            let v = node.borrow();
            tra(&v.left, ret);
            tra(&v.right, ret);
            ret.push(v.val);
        }
    }
    let mut ret: Vec<i32> = vec![];
    tra(&root, &mut ret);
    ret
}


#[cfg(test)]
mod tests {
    use crate::tree_from_vec;
    use super::*;

    #[test]
    fn it_works() {
        let tree = tree_from_vec![1,null,2,3];
        let result = postorder_traversal(tree);
        assert_eq!(result, vec![3,2,1]);
    }
}

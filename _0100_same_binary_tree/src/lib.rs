use std::cell::RefCell;
use std::rc::Rc;

/// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
/// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut result = true;
    visit_node(&p, &q, &mut result);
    result
}

pub fn visit_node(tree1: &Option<Rc<RefCell<TreeNode>>>, tree2: &Option<Rc<RefCell<TreeNode>>>, result: &mut bool) {
    if tree1.is_some() && tree2.is_some() {
            if &tree1.as_ref().unwrap().as_ref().borrow().val != &tree2.as_ref().unwrap().as_ref().borrow().val {
                *result =  false;
            }
            visit_node(&tree1.as_ref().unwrap().borrow().left, &tree2.as_ref().unwrap().borrow().left, result);
            visit_node(&tree1.as_ref().unwrap().borrow().right, &tree2.as_ref().unwrap().borrow().right, result);
    } else if tree1.is_none() && tree2.is_some() || tree1.is_some() && tree2.is_none() {
        *result = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
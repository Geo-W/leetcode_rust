use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut found_none = false;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();
            for child in [&node.left, &node.right] {
                match child {
                    None => found_none = true,
                    Some(v) => {
                        queue.push_back(v.clone());
                        if found_none {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(is_complete_tree(tree_from_vec![1, 2, 3, 4, 5, 6]), true);
    }

    #[test]
    fn b() {
        assert_eq!(
            is_complete_tree(tree_from_vec![1, 2, 3, 4, 5, null, 7]),
            false
        );
    }
}

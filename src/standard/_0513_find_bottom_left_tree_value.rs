use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut deque = VecDeque::new();
    let mut ret = 0;
    deque.push_back(root.unwrap());
    while !deque.is_empty() {
        ret = deque.front().unwrap().borrow().val;
        for _ in 0..deque.len() {
            let rc = deque.pop_front().unwrap();
            let node = rc.borrow();
            if let Some(v) = &node.left {
                deque.push_back(v.clone());
            }
            if let Some(v) = &node.right {
                deque.push_back(v.clone());
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(find_bottom_left_value(tree_from_vec![2, 1, 3]), 1);
    }

    #[test]
    fn b() {
        assert_eq!(
            find_bottom_left_value(tree_from_vec![1, 2, 3, 4, null, 5, 6, null, null, 7]),
            7
        );
    }
}

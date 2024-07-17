use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut deque = VecDeque::new();
    let mut ret = vec![];
    if root.is_none() {
        return ret;
    }
    deque.push_back(root.unwrap());
    while !deque.is_empty() {
        let mut tmp = i32::MIN;
        for _ in 0..deque.len() {
            let rc = deque.pop_front().unwrap();
            let node = rc.borrow();
            if let Some(v) = &node.left {
                deque.push_back(v.clone());
            }
            if let Some(v) = &node.right {
                deque.push_back(v.clone());
            }
            tmp = max(tmp, node.val);
        }
        ret.push(tmp);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(
            largest_values(tree_from_vec![1, 3, 2, 5, 3, null, 9]),
            vec![1, 3, 9]
        );
    }
}

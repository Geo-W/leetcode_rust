use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut tmp = VecDeque::new();
    fn recur(tmp: &mut VecDeque<Rc<RefCell<TreeNode>>>, odd_level: bool) {
        if !tmp.is_empty() {
            let len = tmp.len();
            if odd_level {
                for i in 0..len / 2 {
                    swap_value(&tmp[i], &tmp[len - 1 - i]);
                }
            }
            for _ in 0..len {
                if let Some(v) = tmp.pop_front() {
                    if let Some(v_left) = v.borrow().left.clone() {
                        tmp.push_back(v_left);
                    }
                    if let Some(v_right) = v.borrow().right.clone() {
                        tmp.push_back(v_right);
                    }
                }
            }
            recur(tmp, !odd_level);
        }
    }
    let r = root.clone();

    tmp.push_front(root.unwrap());
    recur(&mut tmp, false);
    r
}

fn swap_value(node1: &Rc<RefCell<TreeNode>>, node2: &Rc<RefCell<TreeNode>>) {
    let mut node1_mut = node1.borrow_mut();
    let mut node2_mut = node2.borrow_mut();
    std::mem::swap(&mut node1_mut.val, &mut node2_mut.val);
}

#[cfg(test)]
mod tests {
    use crate::tree_from_vec;

    use super::*;

    #[test]
    fn a() {
        let a = tree_from_vec![2, 3, 5, 8, 13, 21, 34];
        reverse_odd_levels(a.clone());
        assert_eq!(a, tree_from_vec![2, 5, 3, 8, 13, 21, 34]);
    }

    #[test]
    fn b() {
        let a = tree_from_vec![7,13,11];
        reverse_odd_levels(a.clone());
        assert_eq!(a, tree_from_vec![7,11,13]);
    }

    #[test]
    fn c() {
        let a = tree_from_vec![0,1,2,0,0,0,0,1,1,1,1,2,2,2,2];
        reverse_odd_levels(a.clone());
        assert_eq!(a, tree_from_vec![0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]);
    }
}

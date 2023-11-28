use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = VecDeque::new();
    let mut arr = vec![];
    fn recur(
        ret: &mut Vec<Vec<i32>>,
        tmp: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        arr: &mut Vec<i32>,
        reversed: bool,
    ) {
        if !tmp.is_empty() {
            for _ in 0..tmp.len() {
                if !reversed {
                    if let Some(v) = tmp.pop_front().unwrap() {
                        arr.push(v.borrow().val);
                        tmp.push_back(v.borrow_mut().left.take());
                        tmp.push_back(v.borrow_mut().right.take());
                    }
                } else {
                    if let Some(v) = tmp.pop_back().unwrap() {
                        arr.push(v.borrow().val);
                        tmp.push_front(v.borrow_mut().right.take());
                        tmp.push_front(v.borrow_mut().left.take());
                    }
                }
            }
            ret.push(arr.clone());
            arr.clear();
            recur(ret, tmp, arr, !reversed);
        }
    }

    tmp.push_front(root);
    recur(&mut ret, &mut tmp, &mut arr, false);
    ret.pop();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree_from_vec, vec_vec};

    #[test]
    fn a() {
        assert_eq!(
            zigzag_level_order(tree_from_vec![3, 9, 20, null, null, 15, 7]),
            vec_vec![[3], [20, 9], [15, 7]]
        );
    }
}

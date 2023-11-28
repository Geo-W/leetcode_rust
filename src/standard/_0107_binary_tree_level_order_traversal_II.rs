use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut tmp = VecDeque::new();
    let mut arr = vec![];
    fn recur(
        ret: &mut Vec<Vec<i32>>,
        tmp: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        arr: &mut Vec<i32>,
    ) {
        if !tmp.is_empty() {
            for i in 0..tmp.len() {
                if let Some(v) = tmp.pop_front().unwrap() {
                    arr.push(v.borrow().val);
                    tmp.push_back(v.borrow_mut().left.take());
                    tmp.push_back(v.borrow_mut().right.take());
                }
            }
            ret.push(arr.clone());
            arr.clear();
            recur(ret, tmp, arr);
        }
    }

    tmp.push_front(root);
    recur(&mut ret, &mut tmp, &mut arr);
    ret.pop();
    ret.reverse();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree_from_vec, vec_vec};

    #[test]
    fn a() {
        assert_eq!(
            level_order_bottom(tree_from_vec![3, 9, 20, null, null, 15, 7]),
            vec_vec![[15, 7], [9, 20], [3]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(level_order_bottom(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn c() {
        assert_eq!(level_order_bottom(tree_from_vec![1]), vec_vec![[1]]);
    }
}

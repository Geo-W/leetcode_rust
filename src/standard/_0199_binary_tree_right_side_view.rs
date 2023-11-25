use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn tra(
        tmp1: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        tmp2: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        tmp: &mut Option<i32>,
        ret: &mut Vec<i32>,
    ) {
        if let Some(node) = tmp1.pop_front().unwrap() {
            let v = Rc::try_unwrap(node).unwrap().into_inner();
            *tmp = Some(v.val);
            tmp2.push_back(v.left);
            tmp2.push_back(v.right);
        }
        if tmp1.is_empty() {
            if !tmp.is_none() {
                ret.push(tmp.unwrap());
                *tmp = None;
            }
            std::mem::swap(tmp1, tmp2);
        }
        if !tmp1.is_empty() {
            tra(tmp1, tmp2, tmp, ret);
        }
    }
    let mut ret = vec![];
    let mut tmp = None;
    let mut vec1 = VecDeque::new();
    vec1.push_back(root);
    let mut vec2 = VecDeque::new();
    tra(&mut vec1, &mut vec2, &mut tmp, &mut ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(
            right_side_view(tree_from_vec![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
    }
}

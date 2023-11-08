use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn tra(
        tmp1: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        tmp2: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        res: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = tmp1.pop_front().unwrap() {
            let v = Rc::try_unwrap(node).unwrap().into_inner();
            res.push(v.val);
            tmp2.push_back(v.left);
            tmp2.push_back(v.right);
        }
        if tmp1.is_empty() {
            if !res.is_empty() {
                ret.push(res.clone());
                res.clear();
            }
            std::mem::swap(tmp1, tmp2);
        }
        if !tmp1.is_empty() {
            tra(tmp1, tmp2, res, ret);
        }
    }
    let mut ret = vec![];
    let mut res = vec![];
    let mut vec1 = VecDeque::new();
    vec1.push_back(root);
    let mut vec2 = VecDeque::new();
    tra(&mut vec1, &mut vec2, &mut res, &mut ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn it_works() {
        let tree = tree_from_vec![3, 9, 20, null, null, 15, 7];
        let result = level_order(tree);
        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn b() {
        assert_eq!(level_order(tree_from_vec![1, 2]), vec![vec![1], vec![2]]);
    }
}

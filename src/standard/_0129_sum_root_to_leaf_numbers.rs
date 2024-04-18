use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ret = 0;
    fn dfs(next: Rc<RefCell<TreeNode>>, ret: &mut i32, cur: &mut Vec<i32>) {
        let mut last_node = true;
        let node = next.borrow();
        cur.push(node.val);
        if let Some(v) = node.left.as_ref() {
            last_node = false;
            dfs(v.clone(), ret, cur);
        }
        if let Some(v) = node.right.as_ref() {
            last_node = false;
            dfs(v.clone(), ret, cur);
        }
        if last_node {
            *ret += cur.iter().fold(0, |cur, nxt| cur * 10 + *nxt);
        }
        cur.pop();
    }
    dfs(root.unwrap(), &mut ret, &mut vec![]);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_from_vec;

    #[test]
    fn a() {
        assert_eq!(sum_numbers(tree_from_vec![1, 2, 3]), 25);
    }

    #[test]
    fn b() {
        assert_eq!(sum_numbers(tree_from_vec![4, 9, 0, 5, 1]), 1026);
    }
}

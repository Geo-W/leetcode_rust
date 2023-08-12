use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::TreeNode;

/// Given the root of a binary tree, return all root-to-leaf paths in any order.
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {

    fn tra(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<String>, cur_path: Vec<i32>) {
        let v = node.as_ref().unwrap().borrow();
        let mut cur_path = cur_path;
        cur_path.push(v.val);

        match (&v.left, &v.right) {
            (Some(_), Some(_)) => {
                tra(&v.left, ret, cur_path.clone());
                tra(&v.right, ret, cur_path);
            }
            (Some(_), None) => {
                tra(&v.left, ret, cur_path);
            }
            (None, Some(_)) => {
                tra(&v.right, ret, cur_path);
            }
            (None, None) => {
                ret.push(cur_path.iter().map(|i|i.to_string()).reduce(|cur,nxt| format!("{}->{}", cur, nxt)).unwrap());
            }
        }
    }
    if let Some(_) = &root {
        let mut ret: Vec<String> = vec![];
        tra(&root, &mut ret, vec![]);
        return ret;
    }


    vec![]
}


#[cfg(test)]
mod tests {
    use crate::standard::_0257_binary_tree_paths::binary_tree_paths;
    use crate::utils::tree::TreeNode;
    use crate::{tree_from_vec, vec_string};

    #[test]
    fn it_works() {
        assert_eq!(binary_tree_paths(tree_from_vec![1,2,3,null,5]),
                vec_string!["1->2->5","1->3"])
    }
}

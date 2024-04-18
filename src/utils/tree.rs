#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(list: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if list.is_empty() {
            return None;
        }

        let root = match list[0] {
            Some(value) => Rc::new(RefCell::new(TreeNode::new(value))),
            None => return None,
        };

        let mut node_queue = std::collections::VecDeque::new();
        node_queue.push_back(root.clone());

        for value in list[1..].chunks(2) {
            // println!("{:?}", value);
            if let Some(cur) = node_queue.pop_front() {
                if let Some(v) = value[0] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(v)));
                    cur.borrow_mut().left = Some(left_child.clone());
                    node_queue.push_back(left_child);

                    // dbg!(&root);
                }
                if value.len() > 1 {
                    if let Some(v) = value[1] {
                        let right_child = Rc::new(RefCell::new(TreeNode::new(v)));
                        cur.borrow_mut().right = Some(right_child.clone());
                        node_queue.push_back(right_child);
                    }
                }
                // println!("root:{:?}", root);
            }
        }

        Some(root)
    }
}

#[macro_export]
macro_rules! tree_from_vec {
    ($($element:expr),*) => {
        TreeNode::from_vec(
            vec![$(stringify!($element)), *].iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<Option<i32>>>()
        )
    };
}

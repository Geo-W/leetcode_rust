#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub(crate) fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn linked_list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut output = None;
    for i in vec.iter().rev() {
        if output.is_none() {
            output = Some(Box::new(ListNode::new(*i)));
        } else {
            output = Some(Box::new(ListNode { val: *i, next: output }));
        }
    }
    output
}

pub fn linked_list_to_vec(ls: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut ls = ls;
    while let Some(v) = ls {
        vec.push(v.val);
        ls = v.next;
    }
    vec
}
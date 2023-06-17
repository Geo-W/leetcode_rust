pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // the initial value in b be better as large as possible.
    // if it equals to the first node, then that node will be skipped.
    // e.g. [0,0,0,0,0], output None
    let mut b = Some(Box::new(ListNode { val: i32::MAX, next: head }));
    let mut a = b.as_mut();
    let mut temp = a.as_mut().unwrap().val;
    while a.is_some() && a.as_mut().unwrap().next.is_some() {
        if temp == a.as_mut().unwrap().next.as_mut().unwrap().val {
            a.as_mut().unwrap().next = a.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            a = a.unwrap().next.as_mut();
            temp = a.as_mut().unwrap().val;
        }
    }
    b.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = delete_duplicates(linked_list_from_vec( vec![1,1,2]));
        assert_eq!(result, linked_list_from_vec(vec![1,2]));
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
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
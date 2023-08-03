/// Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut b = Some(Box::new(ListNode { val: i32::MAX, next: head }));
    let mut a = b.as_mut();
    while a.is_some() && a.as_mut().unwrap().next.is_some() {
        if a.as_mut().unwrap().next.as_mut().unwrap().val == val {
            a.as_mut().unwrap().next = a.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            a = a.unwrap().next.as_mut();
        }
    }
    b.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = remove_elements(linked_list_from_vec(vec![1, 2, 3, 3, 4, 4, 5]), 5);
        assert_eq!(result, linked_list_from_vec(vec![1, 2, 3, 3, 4, 4]));
    }

    #[test]
    fn a() {
        let result = remove_elements(linked_list_from_vec(vec![1, 1, 1, 2, 3]), 1);
        assert_eq!(result, linked_list_from_vec(vec![2, 3]));
    }

    #[test]
    fn c() {
        let result = remove_elements(linked_list_from_vec(vec![1, 2, 6, 3, 4, 5, 6]), 6);
        assert_eq!(result, linked_list_from_vec(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn da() {
        let result = remove_elements(linked_list_from_vec(vec![1, 1, 1]), 1);
        assert_eq!(result, linked_list_from_vec(vec![]));
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
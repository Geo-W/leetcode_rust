/// Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut b = Some(Box::new(ListNode { next: head, val: i32::MAX }));
    let mut l = b.as_mut();
    while l.as_mut().unwrap().next.is_some() && l.as_mut().unwrap().next.as_mut().unwrap().next.is_some() {
        let last = l.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.take();
        let second = l.as_mut().unwrap().next.as_mut().unwrap().next.take();
        let first = l.as_mut().unwrap().next.take();
        l.as_mut().unwrap().next = second;
        l.as_mut().unwrap().next.as_mut().unwrap().next = first;
        l.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = last;
        l = l.unwrap().next.as_mut().unwrap().next.as_mut();
    }

    b.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = swap_pairs(linked_list_from_vec(vec![1, 2, 3, 3, 4, 4, 5]));
        assert_eq!(result, linked_list_from_vec(vec![2,1,3,3,4,4,5]));
    }

    #[test]
    fn a() {
        let result = swap_pairs(linked_list_from_vec(vec![]));
        assert_eq!(result, linked_list_from_vec(vec![]));
    }

    #[test]
    fn c() {
        let result = swap_pairs(linked_list_from_vec(vec![1, 2, 3, 3, 5]));
        assert_eq!(result, linked_list_from_vec(vec![2,1,3,3,5]));
    }

    #[test]
    fn da() {
        let result = swap_pairs(linked_list_from_vec(vec![1, 2, 1]));
        assert_eq!(result, linked_list_from_vec(vec![2,1,1]));
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
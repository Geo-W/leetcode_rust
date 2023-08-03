pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut b = Some(Box::new(ListNode { next: head, val: i32::MAX }));
    let mut l = b.as_mut();
    let mut cur = i32::MAX;
    while l.as_mut().unwrap().next.is_some() && l.as_mut().unwrap().next.as_mut().unwrap().next.is_some() {
        if l.as_mut().unwrap().next.as_mut().unwrap().val == l.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().val {
            cur = l.as_mut().unwrap().next.as_mut().unwrap().val;
            l.as_mut().unwrap().next = l.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else if l.as_mut().unwrap().next.as_mut().unwrap().val == cur {
            l.as_mut().unwrap().next = l.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            l = l.unwrap().next.as_mut();
        }
    }
    if l.as_mut().unwrap().next.is_some() && l.as_mut().unwrap().next.as_mut().unwrap().val == cur {
        l.as_mut().unwrap().next = l.as_mut().unwrap().next.as_mut().unwrap().next.take();
    }


    b.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = delete_duplicates(linked_list_from_vec(vec![1, 2, 3, 3, 4, 4, 5]));
        assert_eq!(result, linked_list_from_vec(vec![1, 2, 5]));
    }

    #[test]
    fn a() {
        let result = delete_duplicates(linked_list_from_vec(vec![1, 1, 1, 2, 3]));
        assert_eq!(result, linked_list_from_vec(vec![2, 3]));
    }

    #[test]
    fn c() {
        let result = delete_duplicates(linked_list_from_vec(vec![1, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5]));
        assert_eq!(result, linked_list_from_vec(vec![1, 2, 5]));
    }

    #[test]
    fn da() {
        let result = delete_duplicates(linked_list_from_vec(vec![1, 1, 1]));
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
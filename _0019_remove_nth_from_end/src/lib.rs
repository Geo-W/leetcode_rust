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

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut result = Some(Box::new(ListNode { next: head, val: 0 }));
    let mut result2 = result.clone();
    let mut slow = result.as_mut();
    let mut fast = result2.as_ref();
    let mut count = 0;
    while count < n + 1 {
        fast = fast.unwrap().next.as_ref();
        count += 1;
    }
    while fast.is_some() {
        fast = fast.unwrap().next.as_ref();
        slow = slow.unwrap().next.as_mut();
    }
    slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
    result.unwrap().next
}

impl ListNode {
    fn remove_next(&mut self) {
        if let Some(node) = &self.next {
            self.next = node.next.to_owned()
        } else {
            self.next = None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let l = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(remove_nth_from_end(l, 2), linked_list_from_vec(vec![1, 2, 3, 5]));
    }

    #[test]
    fn b() {
        assert_eq!(remove_nth_from_end(
            linked_list_from_vec(vec![1]), 1,
        ), linked_list_from_vec(vec![]))
    }

    #[test]
    fn c() {
        assert_eq!(remove_nth_from_end(
            linked_list_from_vec(vec![1, 2]), 1,
        ), linked_list_from_vec(vec![1]))
    }
}



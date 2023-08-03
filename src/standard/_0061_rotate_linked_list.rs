/// Given the head of a linked list, rotate the list to the right by k places.
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let len_fn = |mut head: &Option<Box<ListNode>>| {
        let mut len = 0;
        while let Some(v) = head {
            len +=1;
            head = &v.next;
        }
        len
    };

    let reconnect = |mut head: Option<Box<ListNode>>,pos: i32| {
        let mut ptr = head.as_mut();
        for _ in 0..(pos) {
            ptr = ptr.unwrap().next.as_mut();
        }
        let mut tail = ptr.unwrap().next.take();
        let mut tail_ptr = tail.as_mut().unwrap().as_mut();
        while tail_ptr.next.is_some(){
            tail_ptr = tail_ptr.next.as_mut().unwrap().as_mut();
        }
        tail_ptr.next = head;
        tail
    };

    let len = len_fn(&head);
    if len > 1 && k % len != 0 {
        reconnect(head, len - k % len - 1)
    } else {
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rotate_right(linked_list_from_vec(vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(result, linked_list_from_vec(vec![4, 5, 1, 2, 3]));
    }

    #[test]
    fn a() {
        let result = rotate_right(linked_list_from_vec(vec![0, 1, 2]), 4);
        assert_eq!(result, linked_list_from_vec(vec![2, 0, 1]));
    }

    #[test]
    fn b() {
        let result = rotate_right(linked_list_from_vec(vec![1]), 4);
        assert_eq!(result, linked_list_from_vec(vec![1]));
    }

    #[test]
    fn c() {
        let result = rotate_right(linked_list_from_vec(vec![]), 4);
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
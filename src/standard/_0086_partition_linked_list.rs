/// Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
/// You should preserve the original relative order of the nodes in each of the two partitions.
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut smaller = ListNode::new(0);
    let mut bigger = ListNode::new(0);

    let mut small_ptr = &mut smaller;
    let mut bigger_ptr = &mut bigger;

    while let Some(mut v) = head{
        head = v.next.take();

        if v.val < x{
            small_ptr.next = Some(v);
            small_ptr = small_ptr.next.as_mut().unwrap();
        } else {
            bigger_ptr.next = Some(v);
            bigger_ptr = bigger_ptr.next.as_mut().unwrap();
        }
    }

    small_ptr.next = bigger.next;

    smaller.next

}

pub fn partition2(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut smaller = Some(Box::new(ListNode { next: None, val: 0 }));
    let mut bigger = Some(Box::new(ListNode { next: head, val: 0 }));
    let mut ptr_small = smaller.as_mut();
    let mut ptr_big = bigger.as_mut();

    while ptr_big.as_mut().unwrap().next.is_some() {
        if ptr_big.as_mut().unwrap().next.as_mut().unwrap().val < x {
            ptr_small.as_mut().unwrap().next = ptr_big.as_mut().unwrap().next.take();
            ptr_small = ptr_small.unwrap().next.as_mut();
            ptr_big.as_mut().unwrap().next = ptr_small.as_mut().unwrap().next.take();
        } else {
            ptr_big = ptr_big.unwrap().next.as_mut();
        }
    }

    ptr_small.as_mut().unwrap().next = bigger.unwrap().next.take();
    smaller.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = partition(linked_list_from_vec(vec![1, 4, 3, 2, 5, 2]), 3);
        assert_eq!(result, linked_list_from_vec(vec![1, 2, 2, 4, 3, 5]));
    }

    #[test]
    fn a() {
        let result = partition(linked_list_from_vec(vec![2, 1]), 2);
        assert_eq!(result, linked_list_from_vec(vec![1, 2]));
    }

    #[test]
    fn b() {
        let result = partition(linked_list_from_vec(vec![2, 1]), 3);
        assert_eq!(result, linked_list_from_vec(vec![2, 1]));
    }

        #[test]
    fn c() {
        let result = partition(linked_list_from_vec(vec![2]), 3);
        assert_eq!(result, linked_list_from_vec(vec![2]));
    }

            #[test]
    fn d() {
        let result = partition(linked_list_from_vec(vec![]), 3);
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
        ListNode {
            next: None,
            val,
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
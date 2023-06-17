/// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = list1;
    let mut l2 = list2;
    let mut result = Some(Box::new(ListNode { val: 0, next: None }));
    let mut pointer = result.as_mut();
    loop {
        match (&mut l1, &mut l2) {
            (None, None) => {
                break;
            }
            (Some(v1), Some(v2)) => {
                if v1.val >= v2.val {
                    pointer.as_mut().unwrap().next = Some(Box::new(ListNode { val: v2.val, next: None }));
                    pointer = pointer.unwrap().next.as_mut();
                    l2 = l2.unwrap().next;
                } else {
                    pointer.as_mut().unwrap().next = Some(Box::new(ListNode { val: v1.val, next: None }));
                    pointer = pointer.unwrap().next.as_mut();
                    l1 = l1.unwrap().next;
                }
            }
            (Some(v1), None) => {
                pointer.as_mut().unwrap().next = Some(Box::new(ListNode { val: v1.val, next: None }));
                pointer = pointer.unwrap().next.as_mut();
                l1 = l1.unwrap().next;
            }
            (None, Some(v2)) => {
                pointer.as_mut().unwrap().next = Some(Box::new(ListNode { val: v2.val, next: None }));
                pointer = pointer.unwrap().next.as_mut();
                l2 = l2.unwrap().next;
            }
        }
    }
    result.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = merge_two_lists(linked_list_from_vec(vec![1, 2, 4]), linked_list_from_vec(vec![1, 3, 4]));
        assert_eq!(result, linked_list_from_vec(vec![1,1,2,3,4,4]));
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

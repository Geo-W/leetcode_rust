use crate::utils::linked_list::ListNode;

/// Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.
/// The first node is considered odd, and the second node is even, and so on.
/// Note that the relative order inside both the even and odd groups should remain as it was in the input.
///4 You must solve the problem in O(1) extra space complexity and O(n) time complexity.
pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd = Some(Box::new(ListNode::new(0)));
    let mut even = Some(Box::new(ListNode::new(0)));

    let mut odd_ptr = odd.as_mut();
    let mut even_ptr = even.as_mut();

    let mut odd_idx = true;

    while head.is_some() {
        if odd_idx {
            odd_ptr.as_mut().unwrap().next = head.take();
            odd_ptr = odd_ptr.unwrap().next.as_mut();
            head = odd_ptr.as_mut().unwrap().next.take();
        } else {
            even_ptr.as_mut().unwrap().next = head.take();
            even_ptr = even_ptr.unwrap().next.as_mut();
            head = even_ptr.as_mut().unwrap().next.take();
        }
        odd_idx = !odd_idx;
    }

    odd_ptr.as_mut().unwrap().next = even.unwrap().next;

    odd.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::linked_list_from_vec;

    #[test]
    fn it_works() {
        let ls = linked_list_from_vec(vec![2, 1, 3, 5, 6, 4, 7]);
        assert_eq!(
            odd_even_list(ls),
            linked_list_from_vec(vec![2, 3, 6, 7, 1, 5, 4])
        );
    }
}

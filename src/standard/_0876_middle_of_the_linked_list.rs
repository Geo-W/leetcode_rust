use crate::utils::linked_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    'outer: loop {
        for _ in 0..2 {
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
            } else {
                break 'outer;
            }
        }
        slow = &slow.as_ref().unwrap().next;
    }
    slow.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::linked_list_from_vec;

    #[test]
    fn a() {
        assert_eq!(
            middle_node(linked_list_from_vec(vec![1, 2, 3, 4, 5])),
            linked_list_from_vec(vec![3, 4, 5])
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            middle_node(linked_list_from_vec(vec![1, 2, 3, 4, 5, 6])),
            linked_list_from_vec(vec![4, 5, 6])
        );
    }
}

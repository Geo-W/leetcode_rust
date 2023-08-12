use crate::utils::linked_list::{ListNode};

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut ret = ListNode::new(0);
    let mut ptr = &mut ret;
    let mut head = head;

    let mut tmp_vec = vec![];
    'outer: loop {
        for _ in 0..k {
            if head.is_some() {
                let mut a = head;
                head = a.as_mut().unwrap().next.take();
                tmp_vec.push(
                    a
                )
            } else {
                break 'outer;
            }
        }

        while let Some(ele) = tmp_vec.pop() {
            ptr.next = ele;
            ptr = ptr.next.as_mut().unwrap();
        }
    }

    for node in tmp_vec.into_iter() {
        ptr.next = node;
        ptr = ptr.next.as_mut().unwrap();
    }

    ret.next
}


#[cfg(test)]
mod tests {
    use crate::utils::linked_list::linked_list_from_vec;
    use super::*;

    #[test]
    fn t1() {
        let result = reverse_k_group(linked_list_from_vec(vec![1, 3, 5, 6, 7]), 3);
        assert_eq!(result,
                   linked_list_from_vec(vec![5, 3, 1, 6, 7])
        );
    }

    #[test]
    fn t2() {
                let result = reverse_k_group(linked_list_from_vec(vec![1,2,3,4,5]), 2);
        assert_eq!(result,
                   linked_list_from_vec(vec![2,1,4,3,5])
        );
    }
}
use crate::utils::linked_list::{linked_list_to_vec, ListNode};

/// You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = linked_list_to_vec(l1).into_iter().rev();
    let mut l2 = linked_list_to_vec(l2).into_iter().rev();
    let mut carry = 0;
    let mut ret: Option<Box<ListNode>> = None;
    loop {
        match (l1.next(), l2.next(), carry) {
            (Some(v1), Some(v2), 0..=999) => {
                let sum = v1 + v2 + carry;
                if sum > 9 {
                    ret = attach(ret, sum - 10);
                    carry = 1;
                } else {
                    ret = attach(ret, sum);
                    carry = 0;
                }
            }
            (Some(v1), None, 0..=999) => {
                let sum = v1 + carry;
                if sum > 9 {
                    ret = attach(ret, sum - 10);
                    carry = 1;
                } else {
                    ret = attach(ret, sum);
                    carry = 0;
                }
            }
            (None, Some(v2), 0..=999) => {
                let sum = v2 + carry;
                if sum > 9 {
                    ret = attach(ret, sum - 10);
                    carry = 1;
                } else {
                    ret = attach(ret, sum);
                    carry = 0;
                }
            }
            (None, None, 1..=999) => {
                ret = attach(ret, carry);
                carry = 0;
            }
            (_, _, _) => { break; }
        }
    }
    ret
}

pub fn attach(mut ls: Option<Box<ListNode>>, i: i32) -> Option<Box<ListNode>> {
    if ls.is_none() {
        ls = Some(Box::new(ListNode::new(i)));
    } else {
        ls = Some(Box::new(ListNode { val: i, next: ls }));
    }
    ls
}


#[cfg(test)]
mod tests {
    use crate::utils::linked_list::linked_list_from_vec;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two_numbers(linked_list_from_vec(vec![7, 2, 4, 3]), linked_list_from_vec(vec![5, 6, 4])),
                   linked_list_from_vec(vec![7, 8, 0, 7]));
    }

    #[test]
    fn a() {
        assert_eq!(add_two_numbers(linked_list_from_vec(vec![5]), linked_list_from_vec(vec![5])),
                   linked_list_from_vec(vec![1, 0]));
    }
}

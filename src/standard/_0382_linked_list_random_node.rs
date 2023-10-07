use rand::Rng;
use crate::utils::linked_list::ListNode;

/// Given a singly linked list, return a random node's value from the linked list. Each node must have the same probability of being chosen.
#[derive(Debug)]
struct Solution {
    ls: Option<Box<ListNode>>,
    length: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut len: usize = 0;
        let mut ptr = head.as_ref();
        while let Some(v) = &ptr.unwrap().next {
            ptr = Some(v);
            len += 1;
        }
        Self {
            ls: head,
            length: len + 1,
        }
    }

    fn get_random(&self) -> i32 {
        let mut idx = rand::thread_rng().gen_range(0, self.length);
        let mut ptr = self.ls.as_ref();
        while idx > 0 {
            ptr = ptr.unwrap().next.as_ref();
            idx -= 1;
        }

        for _ in (0..rand::thread_rng().gen_range(0, self.length)) {
            ptr = ptr.unwrap().next.as_ref();
            idx -= 1;
        }
        ptr.unwrap().val
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::linked_list_from_vec;
    use super::*;

    #[test]
    fn a() {
        let s = Solution::new(
            linked_list_from_vec(vec![1, 2, 3])
        );
        s.get_random();
        s.get_random();
        s.get_random();
        s.get_random();
    }

    #[test]
    fn b() {
        let s = Solution::new(
            linked_list_from_vec(vec![1])
        );
        s.get_random();
        s.get_random();
        s.get_random();
    }
}
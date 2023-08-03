pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut b = Some(Box::new(ListNode { next: head, val: i32::MAX }));
    let mut l = b.as_mut();
    for i in 1..left{
        l = l.unwrap().next.as_mut();
    }
    let mut following = l.unwrap().next.take();
    // println!("{:?}", following);
    let mut last = following.as_mut();
    for _ in 0..right-left{
        last = last.unwrap().next.as_mut();
    }
    let last = last.unwrap().next.take();

    following = reverse_list(following);
    // println!("{:?}", following);
    l = b.as_mut();
        for i in 1..left{
        l = l.unwrap().next.as_mut();
    }
    l.unwrap().next = following;
    l = b.as_mut();
    while l.as_mut().unwrap().next.is_some(){
        l = l.unwrap().next.as_mut();
    }
    l.unwrap().next = last;


    b.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = reverse_between(linked_list_from_vec(vec![1,2,3,4,5]),2,4);
        assert_eq!(result, linked_list_from_vec(vec![1,4,3,2,5]));
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

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut a = head.as_mut();
    let mut result: Option<Box<ListNode>> = None;
    while a.is_some() {
        // println!("{:?}", a);
        match result {
            None => {
                result = Some(Box::new(ListNode::new(a.as_mut().unwrap().val)));
            }
            Some(val) => {
                result = Some(Box::new(ListNode { val: a.as_mut().unwrap().val, next: Some(val) }))
            }
            _ => {}
        }
        a = a.unwrap().next.as_mut();
    }
    // println!("{:?}", result);
    result
}
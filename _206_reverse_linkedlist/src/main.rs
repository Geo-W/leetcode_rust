#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val: val }
    }
    fn pushback(&mut self) {}
}


fn main() {
    let e = Some(Box::new(ListNode::new(1)));
    let d = Some(Box::new(ListNode { val: 2, next: e }));
    let f = Some(Box::new(ListNode { val: 3, next: d }));
    let g = Some(Box::new(ListNode { val: 4, next: f }));
    let mut l1 = Some(Box::new(ListNode { val: 5, next: g }));
    reverse_list(l1);
}


pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut a = head.as_mut();
    let mut result: Option<Box<ListNode>> = None;
    while a.is_some() {
        println!("{:?}", a);
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
    println!("{:?}", result);
    result
}
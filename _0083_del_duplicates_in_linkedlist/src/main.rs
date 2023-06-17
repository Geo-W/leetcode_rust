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
    let e = Some(Box::new(ListNode::new(0)));
    let d = Some(Box::new(ListNode { val: 0, next: e }));
    let f = Some(Box::new(ListNode { val: 0, next: d }));
    let g = Some(Box::new(ListNode { val: 0, next: f }));
    let mut l1 = Some(Box::new(ListNode { val: 0, next: g }));

    // let mut l1 = Some(Box::new(ListNode::new(5)));
    println!("{:?}", delete_duplicates(l1));

    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // the initial value in b be better as large as possible.
        // if it equals to the first node, then that node will be skipped.
        // e.g. [0,0,0,0,0], output None
        let mut b = Some(Box::new(ListNode { val: 99999, next: head }));
        let mut a = b.as_mut();
        let mut temp = a.as_mut().unwrap().val;
        while a.is_some() && a.as_mut().unwrap().next.is_some() {
            if temp == a.as_mut().unwrap().next.as_mut().unwrap().val {
                a.as_mut().unwrap().next = a.as_mut().unwrap().next.as_mut().unwrap().next.take();
                println!("a Stop：{:?}", a);
            } else {
                a = a.unwrap().next.as_mut();
                println!("a Onward：{:?}", a);
                temp = a.as_mut().unwrap().val;
                println!("temp：{:?}", temp);
            }
        }
        b.unwrap().next
    }
}
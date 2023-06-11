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
    let e = Some(Box::new(ListNode::new(3)));
    let d = Some(Box::new(ListNode { val: 4, next: e }));
    let mut l1 = Some(Box::new(ListNode { val: 2, next: d }));
    let c = Some(Box::new(ListNode::new(4)));
    let b = Some(Box::new(ListNode { val: 6, next: c }));
    let mut l2 = Some(Box::new(ListNode { val: 5, next: b }));

    // let e = Some(Box::new(ListNode::new(9)));
    // let d = Some(Box::new(ListNode { val: 9, next: e }));
    // let mut l1 = Some(Box::new(ListNode { val: 9, next: d }));
    // let c = Some(Box::new(ListNode::new(9)));
    // let b = Some(Box::new(ListNode { val: 9, next: c }));
    // let mut l2 = Some(Box::new(ListNode { val: 9, next: b }));


    // let mut l1 = Box::new(ListNode { val: 0, next: l1 });
    let mut a = l1.as_mut();
    // (while a.is_some() {
    //     a = a.unwrap().next.as_mut();
    // });
    // *a = Some(Box::new(ListNode { val: 9, next: None }));

    println!("{:?}", add_two_numbers3(l1, l2));
}


pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut add_carry = 0;
    // let mut a = Some(Box::new(ListNode { val: 0, next: l1 }));
    // let mut b = Some(Box::new(ListNode { val: 0, next: l2 }));
    let mut a = l1.as_mut();
    let mut b = l2.as_mut();
    let mut tail = Box::new(ListNode { val: 1, next: None });
    loop {
        match (&mut a, &mut b, add_carry) {
            (Some(value1), Some(value2), 0) => {
                let result = value1.val + value2.val;
                if result < 10 {
                    value1.as_mut().val = result;
                    println!("{:?}", value1.val);
                } else {
                    value1.as_mut().val = result - 10;
                    add_carry = 1;
                }
                a = a.unwrap().next.as_mut();
                b = b.unwrap().next.as_mut();
            }
            (Some(value1), Some(value2), 1..=9999999) => {
                let result = value1.val + value2.val + add_carry;
                if result < 10 {
                    value1.as_mut().val = result;
                    println!("{:?}", value1.val);
                } else {
                    value1.as_mut().val = result - 10;
                    add_carry = 1;
                }
                a = a.unwrap().next.as_mut();
                b = b.unwrap().next.as_mut();
            }
            (Some(_), None, 0) => {
                a = a.unwrap().next.as_mut();
            }
            (None, Some(_), 0) => { b = b.unwrap().next.as_mut(); }
            (Some(value1), None, 1) => {
                let result = value1.val + add_carry;
                if result < 10 {
                    value1.as_mut().val = result;
                } else {
                    value1.as_mut().val = result - 10;
                    add_carry = 1;
                }
                a = a.unwrap().next.as_mut();
            }
            (None, Some(value2), 1) => {
                let result = value2.val + add_carry;
                if result < 10 {
                    value2.as_mut().val = result;
                } else {
                    value2.as_mut().val = result - 10;
                    add_carry = 1;
                }
                b = b.unwrap().next.as_mut();
            }
            (None, None, 1) => {
                println!("1 来了");
                add_carry = 0;
                // a = Some(&mut tail);
                // return l1
            }
            (_, _, _) => { break; }
        }
    }
    let mut binding = Box::new(ListNode { val: 1, next: None });
    // *a = Some(&mut binding);
    println!("{:?}", a);
    println!("{:?}", l1);

    l1
}

// pub fn add_two_numbers2(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut add_carry = 0;
//     // let mut a = Some(Box::new(ListNode { val: 0, next: l1 }));
//     // let mut b = Some(Box::new(ListNode { val: 0, next: l2 }));
//     let mut l1 = Box::new(ListNode { val: 0, next: l1 });
//
//     let mut l2 = Box::new(ListNode { val: 0, next: l2 });
//
//     let mut a = l1.as_mut();
//     let mut b = l2.as_mut();
//     let mut tail = Box::new(ListNode { val: 1, next: None });
//     // let count = 0;
//     loop {
//         match (&mut a.next, &mut b.next, add_carry) {
//             (Some(value1), Some(value2), 0) => {
//                 let result = value1.val + value2.val;
//                 if result < 10 {
//                     value1.as_mut().val = result;
//                     println!("{:?}", value1.val);
//                 } else {
//                     value1.as_mut().val = result - 10;
//                     add_carry = 1;
//                 }
//                 a = a.next.as_mut().unwrap();
//                 b = b.next.as_mut().unwrap();
//             }
//             (Some(value1), Some(value2), 1..=9999999) => {
//                 let result = value1.val + value2.val + add_carry;
//                 if result < 10 {
//                     value1.as_mut().val = result;
//                     println!("{:?}", value1.val);
//                 } else {
//                     value1.as_mut().val = result - 10;
//                     add_carry = 1;
//                 }
//                 a = a.next.as_mut().unwrap();
//                 b = b.next.as_mut().unwrap();
//             }
//             (Some(_), None, 0) => {
//                 a = a.next.as_mut().unwrap();
//             }
//             (None, Some(_), 0) => {
//                 b = b.next.as_mut().unwrap();
//                 ;
//             }
//             (Some(value1), None, 1) => {
//                 let result = value1.val + add_carry;
//                 if result < 10 {
//                     value1.as_mut().val = result;
//                 } else {
//                     value1.as_mut().val = result - 10;
//                     add_carry = 1;
//                 }
//                 a = a.next.as_mut().unwrap();
//             }
//             (None, Some(value2), 1) => {
//                 let result = value2.val + add_carry;
//                 if result < 10 {
//                     value2.as_mut().val = result;
//                 } else {
//                     value2.as_mut().val = result - 10;
//                     add_carry = 1;
//                 }
//                 b = b.next.as_mut().unwrap();
//             }
//             (None, None, 1) => {
//                 // 虽然下个节点是None，但是当前节点还是有数据，用传统a.next这种方式不可行
//                 println!("1 来了");
//                 add_carry = 0;
//                 a.next = Some(Box::new(ListNode { val: 1, next: None }));
//
//                 // return l1
//             }
//             (_, _, _) => { break; }
//         }
//     }
//     // *a = Some(&mut binding);
//     println!("{:?}", a);
//     println!("{:?}", l1);
//
//     l1.next
// }

fn calc(value1: &i32, value2: &i32, add_carry: &mut i32) -> i32 {
    if value1 + value2 + *add_carry < 10 {
        let a = value1 + value2 + *add_carry;
        *add_carry = 0;
        a
    } else {
        let a =value1 + value2 + *add_carry - 10;
        *add_carry = 1;
        a
    }
}

pub fn add_two_numbers3(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut add_carry = 0;
    let mut a = l1.as_mut();
    let mut b = l2.as_mut();
    let mut results: Option<Box<ListNode>> = None;
    let mut ptr = &mut results;
    loop {
        match (&mut a, &mut b, add_carry) {
            (Some(value1), Some(value2), 0) => {
                let result = calc(&value1.val, &value2.val, &mut add_carry);
                *ptr = Some(Box::new(ListNode::new(result)));
                ptr = &mut ptr.as_mut().unwrap().next;
                a = a.unwrap().next.as_mut();
                b = b.unwrap().next.as_mut();
            }
            (Some(value1), Some(value2), 1..=9999999) => {
                let result = calc(&value1.val, &value2.val, &mut add_carry);
                *ptr = Some(Box::new(ListNode::new(result)));
                ptr = &mut ptr.as_mut().unwrap().next;
                a = a.unwrap().next.as_mut();
                b = b.unwrap().next.as_mut();
            }
            (Some(value1), None, 0) => {
                *ptr = Some(Box::new(ListNode::new(value1.val)));
                ptr = &mut ptr.as_mut().unwrap().next;
                a = a.unwrap().next.as_mut();
            }
            (None, Some(value2), 0) => {
                *ptr = Some(Box::new(ListNode::new(value2.val)));
                ptr = &mut ptr.as_mut().unwrap().next;
                b = b.unwrap().next.as_mut();
            }
            (Some(value1), None, 1) => {
                let result = calc(&value1.val, &0, &mut add_carry);
                *ptr = Some(Box::new(ListNode::new(result)));
                ptr = &mut ptr.as_mut().unwrap().next;
                a = a.unwrap().next.as_mut();
            }
            (None, Some(value2), 1) => {
                let result = calc(&0, &value2.val, &mut add_carry);
                *ptr = Some(Box::new(ListNode::new(result)));
                ptr = &mut ptr.as_mut().unwrap().next;
                b = b.unwrap().next.as_mut();
            }
            (None, None, 1) => {
                *ptr = Some(Box::new(ListNode::new(1)));
                ptr = &mut ptr.as_mut().unwrap().next;
                add_carry = 0;
            }
            (_, _, _) => { break; }
        }
    }
    println!("{:?}", a);
    println!("{:?}", l1);
    println!("result: {:?}", results);
    results
}

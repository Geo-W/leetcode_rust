use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let mut left_heap = costs
        .iter()
        .take(candidates as usize)
        .enumerate()
        .map(|x| Reverse((*x.1, x.0)))
        .collect::<BinaryHeap<_>>();

    let mut right_heap = costs[(candidates as usize)..costs.len()]
        .iter()
        .enumerate()
        .rev()
        .take(candidates as usize)
        .map(|x| Reverse((*x.1, x.0 + candidates as usize)))
        .collect::<BinaryHeap<_>>();

    let mut ret = 0;
    let mut left_ptr = candidates as usize;
    let mut right_ptr = if costs.len() <= candidates as usize * 2 {
        candidates as usize - 1
    } else {
        costs.len() - candidates as usize - 1
    };

    for _ in 0..k {
        match (left_heap.peek(), right_heap.peek()) {
            (Some(l), Some(r)) => {
                if l >= r {
                    ret += l.0 .0 as i64;
                    left_heap.pop();
                    if right_ptr >= left_ptr {
                        left_heap.push(Reverse((costs[left_ptr], left_ptr)));
                        left_ptr += 1;
                    }
                } else {
                    ret += r.0 .0 as i64;
                    right_heap.pop();
                    if right_ptr >= left_ptr {
                        right_heap.push(Reverse((costs[right_ptr], right_ptr)));
                        right_ptr -= 1;
                    }
                }
            }
            (Some(l), None) => {
                ret += l.0 .0 as i64;
                left_heap.pop();
            }
            (None, Some(r)) => {
                ret += r.0 .0 as i64;
                right_heap.pop();
            }
            _ => unimplemented!(),
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
    }

    #[test]
    fn b() {
        assert_eq!(total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }

    #[test]
    fn c() {
        assert_eq!(
            total_cost(vec![57, 33, 26, 76, 14, 67, 24, 90, 72, 37, 30], 11, 2),
            526
        );
    }
}

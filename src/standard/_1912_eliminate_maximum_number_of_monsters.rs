use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut h = dist.into_iter().zip(speed.into_iter()).map(|(di, sp)| {
        Reverse(
            if di % sp > 0 {
                di / sp + 1
            } else {
                di / sp
            }
        )
    }).collect::<BinaryHeap<Reverse<i32>>>();
    let mut min = 0;
    while let Some(el) = h.pop() {
        if el == Reverse(min) {
            break;
        } else {
            min +=1;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
    }

    #[test]
    fn b() {
        assert_eq!(eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]), 1);
    }
}
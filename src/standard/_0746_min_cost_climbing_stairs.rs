use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut first = cost[0];
    let mut second = cost[1];
    let mut tmp = 0;
    for idx in 2..cost.len() {
        tmp = min(first, second) + cost[idx];
        first = second;
        second = tmp;
    }
    min(first, second)
}

// f(1) = 1
// f(2) = 2
// f(3) = min(f(1), f(2)) + 3
// f(4) = min(f(2), f(3)) + 4

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn b() {
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}

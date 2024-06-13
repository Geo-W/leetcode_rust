pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let dev = (0..gas.len())
        .map(|idx| gas[idx] - cost[idx])
        .collect::<Vec<_>>();
    let mut cal = 0;
    let mut crossing = 0; // stop checking if checked station > 1 loop
    let mut start_idx = 0;
    'outer: while start_idx < gas.len() && crossing < gas.len() {
        if dev[start_idx] == 0 && dev.len() != 1 {
            start_idx += 1;
            continue;
        }
        for idx in start_idx..gas.len() {
            cal += dev[idx];
            if cal < 0 {
                cal = 0;
                crossing += idx - start_idx + 1;

                // if cannot arrive this station, means the previous sum < 0; using next station as start station to check.
                start_idx = idx + 1;
                continue 'outer;
            }
        }
        for idx in 0..start_idx {
            cal += dev[idx];
            if cal < 0 {
                cal = 0;
                crossing += idx - 0 + 1;
                start_idx = idx + 1;
                continue 'outer;
            }
        }
        return start_idx as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            can_complete_circuit(vec![2, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0]),
            0
        );
    }

    #[test]
    fn c() {
        assert_eq!(can_complete_circuit(vec![2], vec![2]), 0);
    }
}

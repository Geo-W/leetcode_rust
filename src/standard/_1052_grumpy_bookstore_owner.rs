pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let minutes = minutes as usize;
    let mut ret = 0;
    match grumpy.iter().position(|x| *x == 1) {
        None => {
            return customers.iter().sum();
        }
        Some(v) => {
            let mut pointer = v;
            let mut initial_sum = customers.iter().enumerate().fold(0, |acc, (idx, item)| {
                return if grumpy[idx] == 0 || (idx >= pointer && idx < pointer + minutes) {
                    acc + *item
                } else {
                    acc
                };
            });
            ret = initial_sum;
            while pointer + minutes < customers.len() {
                if grumpy[pointer] == 1 {
                    initial_sum -= customers[pointer];
                }
                pointer += 1;
                if grumpy[pointer + minutes - 1] == 1 {
                    initial_sum += customers[pointer + minutes - 1];
                }
                ret = std::cmp::max(ret, initial_sum);
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }
}

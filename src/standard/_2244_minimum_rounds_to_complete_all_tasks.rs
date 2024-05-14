pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let map = tasks
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            return acc;
        });

    fn calc(i: i32) -> i32 {
        match i % 3 {
            0 => i / 3,
            2 => i / 3 + 1,
            1 => {
                if i == 1 {
                    -1
                } else {
                    (i - 4) / 3 + 2
                }
            }
            _ => unreachable!(),
        }
    }

    map.into_iter().map(|x| x.1).fold(0, |mut acc, x| {
        return if acc == -1 || calc(x) == -1 {
            -1
        } else {
            acc + calc(x)
        };
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
    }
}

pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut init_step = (k - 1) as usize;
    let mut vec = vec![0; n as usize];
    vec[0] = 1;

    let mut it = (1..=n as usize).cycle().skip(1);

    loop {
        let i = it.nth(init_step).unwrap();
        init_step += k as usize;
        if vec[i - 1] == 1 {
            break;
        } else {
            vec[i - 1] = 1;
        }
    }

    vec.into_iter().enumerate().filter(|&(idx, i)| i == 0).map(|(idx, i)| (idx+1) as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(circular_game_losers(5, 2), vec![4, 5]);
    }
}
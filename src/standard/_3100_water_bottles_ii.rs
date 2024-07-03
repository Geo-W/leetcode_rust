pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut ret = 0;
    let mut empty_bottles = 0;
    let mut full_bottles = num_bottles;
    let mut num_exchange = num_exchange;
    while full_bottles + empty_bottles >= num_exchange {
        empty_bottles += full_bottles;
        ret += full_bottles;
        full_bottles = 0;
        empty_bottles -= num_exchange;
        full_bottles += 1;
        num_exchange += 1;
    }

    ret + full_bottles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_bottles_drunk(13, 6), 15);
    }
}

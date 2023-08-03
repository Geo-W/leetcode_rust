/// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.
pub fn add_digits(num: i32) -> i32 {
    recursive_add(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_digits(38), 2);
    }

        #[test]
    fn a() {
        assert_eq!(add_digits(6666), 6);
    }
}

fn recursive_add(num: i32) -> i32 {
    let mut new = reform(num);
    if new > 10 {
        new = recursive_add(new);
    }
    new
}

fn reform(num: i32) -> i32 {
    let mut num = num;
    let mut sum = 0;
    while num > 10 {
        sum += num % 10;
        num = num / 10;
    }
    sum + num
}
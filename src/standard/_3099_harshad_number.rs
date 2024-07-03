pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut ret = 0;
    let mut x2 = x;
    while x2 > 0 {
        ret += x2 % 10;
        x2 /= 10;
    }
    if x % ret != 0 {
        -1
    } else {
        ret
    }
}

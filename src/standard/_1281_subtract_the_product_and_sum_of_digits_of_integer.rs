/// Given an integer number n, return the difference between the product of its digits and the sum of its digits.
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut product = 1;
    let mut sum = 0;

    let mut n = n;
    while n > 0 {
        product *= n % 10;
        sum += n % 10;
        n /= 10;
    }

    product - sum
}


#[cfg(test)]
mod tests {
    use crate::standard::_1281_subtract_the_product_and_sum_of_digits_of_integer::subtract_product_and_sum;

    #[test]
    fn it_works() {
        assert_eq!(subtract_product_and_sum(4421), 21)
    }
}

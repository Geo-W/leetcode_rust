pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
    if k <= num_ones {
        k
    } else if k <= num_ones + num_zeros {
        num_ones
    } else {
        num_ones - (k - num_ones - num_zeros)
    }
}


pub fn k_items_with_maximum_sum2(
    num_ones: i32,
    num_zeros: i32,
    num_neg_ones: i32,
    mut k: i32,
) -> i32 {
    let mut sum = 0;
    let vec: Vec<Box<dyn Fn(i32, i32, &mut i32) -> i32>> = vec![
        Box::new(|sum: i32, i: i32, k: &mut i32|
            if *k <= i {
                let a = sum + *k;
                *k -= i;
                a
            } else {
                let a = sum + i;
                *k -= i;
                a
            }),
        Box::new(|sum: i32, i: i32, k: &mut i32| {
            *k -= i;
            sum
        }),
        Box::new(|sum: i32, i: i32, k: &mut i32| {
            if *k <= i {
                let a = sum - *k;
                *k -= i;
                a
            } else {
                let a = sum - i;
                *k -= i;
                a
            }
        }),
    ];
    for (i, func) in [num_ones, num_zeros, num_neg_ones].iter().zip(vec.iter()) {
        if k > 0 {
            dbg!(k,sum,i);
            sum = (func)(sum, *i, &mut k);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = k_items_with_maximum_sum(3, 2, 0, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn a() {
        let result = k_items_with_maximum_sum(6, 6, 6, 13);
        assert_eq!(result, 5);
    }
}

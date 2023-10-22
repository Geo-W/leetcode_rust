pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut sorted_s = satisfaction;
    sorted_s.sort();
    let mut sum_tmp = 0;
    let mut ret = 0;
    for i in sorted_s.iter().rev() {
        if *i >= 0 {
            sum_tmp += i;
            ret += sum_tmp;
        } else {
            if -(*i) <= sum_tmp {
                sum_tmp += i;
                ret += sum_tmp;
            }
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
    }

    #[test]
    fn b() {
        assert_eq!(max_satisfaction(vec![4, 3, 2]), 20);
    }

    #[test]
    fn c() {
        assert_eq!(max_satisfaction(vec![-1, -4, -5]), 0);
    }
}
use std::cmp::Ordering;

/// Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.
/// The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.
/// Citations is sorted in ascending order, write an algorithm that runs in logarithmic time.
pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut ptr = citations.len() / 2;
    let mut lower = 0;
    let mut upper = citations.len() - 1;
    let mut slice = &citations[lower..=upper];
    loop {
        match upper.cmp(&lower) {
            Ordering::Less => {
                break;
            }
            Ordering::Equal => {
                return if slice[0] >= (citations.len() - lower) as i32 {
                    (citations.len() - lower) as i32
                } else {
                    0
                };
            }
            Ordering::Greater => {
                if citations[ptr] < (citations.len() - ptr) as i32 {
                    lower = ptr + 1;
                    slice = &citations[lower..=upper];
                } else {
                    upper = ptr;
                    slice = &citations[lower..=upper];
                }
                ptr = (lower + upper) / 2;
            }
        }
    }
    (citations.len() - lower) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(h_index(vec![1, 1, 3]), 1);
    }

    #[test]
    fn c() {
        assert_eq!(h_index(vec![11, 15]), 2);
    }

    #[test]
    fn d() {
        assert_eq!(h_index(vec![100]), 1);
    }

    #[test]
    fn e() {
        assert_eq!(h_index(vec![0]), 0);
    }

    #[test]
    fn f() {
        assert_eq!(h_index(vec![0, 0, 0]), 0);
    }
}
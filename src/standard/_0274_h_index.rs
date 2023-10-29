/// Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.
/// The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.
pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort_by(|x, y| y.cmp(x));
    let mut ret = 0;
    for (i, x) in citations.iter().enumerate() {
        if *x >= (i + 1) as i32 {
            ret +=1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(h_index(vec![1, 3, 1]), 1);
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
        assert_eq!(h_index(vec![0, 0, 0]), 0);
    }
}
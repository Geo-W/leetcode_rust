pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    vec![
        nums1
            .iter()
            .map(|x| if { nums2.contains(x) } { 1 } else { 0 })
            .reduce(|acc, x| acc + x)
            .unwrap(),
        nums2
            .iter()
            .map(|x| if { nums1.contains(x) } { 1 } else { 0 })
            .reduce(|acc, x| acc + x)
            .unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            find_intersection_values(vec![2, 3, 2], vec![1, 2]),
            vec![2, 1]
        );
    }
}

/// Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the kth smallest element in the matrix.
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut ret = matrix.into_iter().flatten().collect::<Vec<i32>>();
    ret.sort();
    ret[(k - 1) as usize]
}

#[cfg(test)]
mod tests {
    use crate::standard::_0378_kth_smallest_element_in_a_sorted_list::kth_smallest;

    #[test]
    fn it_works() {
        assert_eq!(kth_smallest(vec![vec![1,5,9],vec![10,11,13],vec![12,13,15]], 8),13)

    }
}

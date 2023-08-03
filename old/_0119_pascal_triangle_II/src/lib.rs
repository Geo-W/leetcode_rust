/// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut ret = vec![0; (row_index + 1) as usize];
    ret[0] = 1;
    for seq in 0..(row_index) {
        ret[(seq + 1) as usize] = ret[seq as usize];
        for i in (1..=seq).rev() {
            ret[i as usize] = ret[i as usize] + ret[(i - 1) as usize];
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_row(1);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn b() {
        let result = get_row(33);
        assert_eq!(
            result,
            vec![
                1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110,
                1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100,
                13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
            ]
        );
    }

    #[test]
    fn a() {
        let result = get_row(5);
        assert_eq!(result, vec![1, 5, 10, 10, 5, 1]);
    }
}

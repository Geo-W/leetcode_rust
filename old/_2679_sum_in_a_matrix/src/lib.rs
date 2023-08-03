/// You are given a 0-indexed 2D integer array nums. Initially, your score is 0. Perform the following operations until the matrix becomes empty:
///     From each row in the matrix, select the largest number and remove it. In the case of a tie, it does not matter which number is chosen.
///     Identify the highest number amongst all those removed in step 1. Add that number to your score.
pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
    nums.iter_mut().for_each(|x| {
        x.sort();
    });
    (0..nums[0].len()).into_iter().fold(0, |acc, column| {
        (0..nums.len()).into_iter().map(|row| nums[row][column])
            .fold(0, |acc, x| std::cmp::max(acc, x)) + acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = matrix_sum(vec![
            vec![7, 2, 1],
            vec![6, 4, 2],
            vec![6, 5, 3],
            vec![3, 2, 1],
        ]);
        assert_eq!(result, 15);
    }

    #[test]
    fn a() {
        assert_eq!(matrix_sum(vec![vec![1]]), 1);
    }
}

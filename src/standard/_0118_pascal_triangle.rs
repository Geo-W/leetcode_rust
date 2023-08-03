/// Given an integer numRows, return the first numRows of Pascal's triangle.
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![1]];
    for i in 0..(num_rows-1) {
        let mut vec = Vec::with_capacity(i as usize);
        vec.push( ret[i as usize][0 as usize]);
        for idx in 1..=i{
            vec.push(ret[i as usize][idx as usize] + ret[i as usize][(idx - 1) as usize]);
        }
        vec.push(ret[i as usize][(i) as usize]);
        ret.push(vec);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate(5);
        assert_eq!(result, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }
}

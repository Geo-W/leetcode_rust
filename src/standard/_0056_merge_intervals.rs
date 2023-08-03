/// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut vec = intervals;
    vec.sort_by_key(|i| i[0]);
    let mut ret = Vec::with_capacity(vec.len());
    ret.push(vec[0].clone());
    vec.into_iter().skip(1).for_each(|item| {
                if item[0] <= ret[ret.len() - 1][1] && item[1] >= ret[ret.len() - 1][1] {
            let new_item = vec![ret[ret.len() - 1][0], item[1]];
            ret.pop().unwrap();
            ret.push(new_item);
        } else if item[0] > ret[ret.len() -1][1]{
            ret.push(item);
        }
    });

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = merge(vec![
            vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18],
        ]);
        assert_eq!(result, vec![
            vec![1, 6], vec![8, 10], vec![15, 18],
        ]);
    }

    #[test]
    fn a() {
        let result = merge(vec![
            vec![1, 4], vec![4, 5],
        ]);
        assert_eq!(result, vec![
            vec![1, 5],
        ]);
    }

        #[test]
    fn b() {
        let result = merge(vec![
            vec![1, 4], vec![0,4]
        ]);
        assert_eq!(result, vec![
            vec![0, 4],
        ]);
    }

            #[test]
    fn c() {
        let result = merge(vec![
            vec![1, 4], vec![2,3]
        ]);
        assert_eq!(result, vec![
            vec![1, 4],
        ]);
    }
}

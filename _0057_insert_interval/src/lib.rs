/// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti.
/// You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
/// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
/// Return intervals after the insertion.
pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 {
        return vec![new_interval];
    }
    let mut a = (find_insertion_point(&intervals, new_interval[0])) as usize;
    intervals.insert(a, new_interval);
    merge(intervals)
}

pub fn find_insertion_point(intervals: &Vec<Vec<i32>>, target: i32) -> i32 {
    let mut left = 0;
    let mut right: i32 = (intervals.len() - 1) as i32;
    let mut current: usize = 0;

    while left < right {
        current = ((right - left) / 2 + left) as usize;
        if intervals[current][0] < target {
            left = current as i32 + 1; //3
        } else if intervals[current][0] > target {
            right = current as i32 - 1; // 3
        } else if intervals[current][0] == target {
            return current as i32;
        }
    }

    if left >= right || left < 0 || right < 0 {
        if intervals[left as usize][0] >= target {
            return left;
        } else {
            return left + 1;
        }
    } else {
        return 0;
    }
}

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
        } else if item[0] > ret[ret.len() - 1][1] {
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
        let result = insert(vec![
            vec![1, 3], vec![6, 9],
        ], vec![2, 5]);
        assert_eq!(result, vec![
            vec![1, 5], vec![6, 9],
        ]);
    }

    #[test]
    fn a() {
        let result = insert(vec![
            vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16],
        ], vec![4, 8]);
        assert_eq!(result, vec![
            vec![1, 2], vec![3, 10], vec![12, 16],
        ]);
    }

    #[test]
    fn b() {
        let result = insert(vec![], vec![2, 5]);
        assert_eq!(result, vec![vec![2, 5]]);
    }
}

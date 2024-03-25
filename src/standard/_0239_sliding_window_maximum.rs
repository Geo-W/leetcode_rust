use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut q = VecDeque::new();
    let mut ret = vec![];

    for (i, num) in nums.iter().enumerate() {
        if i + 1 > k as usize {
            if let Some(v) = q.front() {
                if nums[i - k as usize] == *v {
                    q.pop_front();
                }
            }
        }

        if q.is_empty() || num <= q.back().unwrap() {
            q.push_back(*num);
        } else {
            while let Some(v) = q.back() {
                if num > v {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(*num);
        }

        if i + 2 > k as usize {
            ret.push(q[0]);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7].to_vec(), 3),
            [3, 3, 5, 5, 6, 7].to_vec()
        );
    }

    #[test]
    fn b() {
        assert_eq!(max_sliding_window([1].to_vec(), 1), [1].to_vec());
    }

    #[test]
    fn c() {
        assert_eq!(max_sliding_window([1, -1].to_vec(), 1), [1, -1].to_vec());
    }

    #[test]
    fn d() {
        assert_eq!(
            max_sliding_window([1, 3, 1, 2, 0, 5].to_vec(), 3),
            [3, 3, 2, 5].to_vec()
        );
    }
}

use std::cmp::max;

pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    let mut dp = vec![Node::default(); nums.len()];
    // first element must be taken.
    dp[0] = if nums[0] % 2 == 0 {
        Node {
            odd: (nums[0] - x) as i64,
            even: nums[0] as i64,
        }
    } else {
        Node {
            odd: nums[0] as i64,
            even: (nums[0] - x) as i64,
        }
    };
    for (idx, &num) in nums.iter().enumerate().skip(1) {
        let mut last = dp[idx - 1];
        if num % 2 == 0 {
            // current element is even, then we calculate max value of even from (add from last max even or add from last max odd).
            // last odd remain untouched.
            last.even = max(last.even + num as i64, last.odd + num as i64 - x as i64);
        } else {
            last.odd = max(last.odd + num as i64, last.even + num as i64 - x as i64);
        }
        dp[idx] = last;
    }

    let last = dp.last().unwrap();
    max(last.odd, last.even)
}

#[derive(Debug, Default, Copy, Clone)]
struct Node {
    odd: i64,
    even: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
    }

    #[test]
    fn b() {
        assert_eq!(
            max_score(
                vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79],
                74
            ),
            470
        );
    }
}

// 2                         ;;; odd(2-5), even: 2 because the first one is a must take.
// 3, max(2, 2+3-5)  2, even ;;; odd max(2+3-5, 3) = 3 even 2
// 6, 2 + 6                  ;;; even: max(3+6-5, 2+6) = 8 odd: 3
// 1, max(8+1-5, 6)          ;;; odd: max(3, 8+1-5=4) = 4, even: 8
// 9,                        ;;; odd: 4+9 = 13, even: max(8+9-5, 8) = 12
// 2,                        ;;; even: 12+2, odd: max(13+2-5)

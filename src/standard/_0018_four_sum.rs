pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    nums.sort();
    let target = target as i64;
    let mut ret = vec![];
    if nums.len() < 4 {
        return ret;
    }

    let mut last_1 = i64::MAX;
    let mut last_2 = i64::MAX;
    let mut last_3 = i64::MAX;
    let mut last_4 = i64::MAX;

    let mut idx_3 = nums.len() - 1;
    let mut idx_2;

    for idx in 0..nums.len() - 2 {
        if nums[idx] == last_1 {
            continue;
        }
        last_1 = nums[idx];
        last_4 = i64::MAX;
        'idx_4_loop: for idx_4 in (3..nums.len()).rev() {
            if idx_4 < idx + 2 {
                break 'idx_4_loop;
            }

            if nums[idx_4] == last_4 {
                continue;
            }

            last_4 = nums[idx_4];
            // initialize pointers for left and right
            idx_3 = idx_4 - 1;
            idx_2 = idx + 1;
            while idx_3 > idx_2 {
                let tmp = nums[idx] + nums[idx_2] + nums[idx_3] + nums[idx_4];
                // if sum > 0, move the right pointer; also check if it is equal to last calculated element.

                if tmp > target || last_3 == nums[idx_3] {
                    last_3 = nums[idx_3];
                    idx_3 -= 1;
                    continue;
                }
                if tmp < target || last_2 == nums[idx_2] {
                    last_2 = nums[idx_2];
                    idx_2 += 1;
                    continue;
                }
                if tmp == target {
                    ret.push(vec![
                        nums[idx] as i32,
                        nums[idx_2] as i32,
                        nums[idx_3] as i32,
                        nums[idx_4] as i32,
                    ]);
                    last_2 = nums[idx_2];
                    idx_2 += 1;
                }
            }
            last_3 = i64::MAX;
            last_2 = i64::MAX;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec_vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
    }

    #[test]
    fn b() {
        assert_eq!(four_sum(vec![2, 2, 2, 2, 2], 8), vec_vec![[2, 2, 2, 2]])
    }

    #[test]
    fn c() {
        assert_eq!(
            four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0),
            vec_vec![[-2, -1, 1, 2], [-1, -1, 1, 1]]
        );
    }

    #[test]
    fn d() {
        assert_eq!(
            four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0),
            vec_vec![
                [-3, -2, 2, 3],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1]
            ]
        );
    }

    #[test]
    fn e() {
        assert_eq!(
            four_sum(vec![-4, -3, -2, -1, 0, 0, 1, 2, 3, 4], 0),
            vec_vec![
                [-4, -3, 3, 4],
                [-4, -2, 2, 4],
                [-4, -1, 1, 4],
                [-4, -1, 2, 3],
                [-4, 0, 0, 4],
                [-4, 0, 1, 3],
                [-3, -2, 1, 4],
                [-3, -2, 2, 3],
                [-3, -1, 0, 4],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1]
            ]
        );
    }

    #[test]
    fn f() {
        assert_eq!(
            four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11),
            vec_vec![[-5, -4, -3, 1]]
        );
    }

    #[test]
    fn g() {
        assert_eq!(
            four_sum(
                vec![-7, -5, 0, 7, 1, 1, -10, -2, 7, 7, -2, -6, 0, -10, -5, 7, -8, 5],
                28
            ),
            vec_vec![[7, 7, 7, 7]]
        );
    }

    #[test]
    fn h() {
        assert_eq!(
            four_sum(
                vec![1000000000, 1000000000, 1000000000, 999999999],
                -294967297,
            ),
            Vec::<Vec<i32>>::new()
        );
    }
}

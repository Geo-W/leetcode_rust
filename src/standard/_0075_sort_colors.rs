pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut cur_2 = nums.len() - 1;
    let mut cur_0 = 0usize;
    let mut idx = 0;
    while idx < nums.len()  {
        match nums[idx] {
            0 => {
                if nums[idx] == nums[cur_0] {
                    idx += 1;
                    cur_0 += 1;
                } else {
                    nums.swap(idx, cur_0);
                    cur_0 += 1;
                }
            }
            1 => {
                if idx + 1 < nums.len() {
                    if nums[idx] > nums[idx + 1] {
                        nums.swap(idx, idx + 1);
                    } else {
                        idx += 1;
                    }
                } else {
                    break;
                }
            }
            2 => {
                if idx >= cur_2 {
                    break;
                } else {
                    nums.swap(cur_2, idx);
                    cur_2 -= 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::standard::_0075_sort_colors::sort_colors;

    #[test]
    fn t1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2])
    }

    #[test]
    fn t2() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn t3() {
        let mut nums = vec![2, 0, 0, 1, 2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn t4() {
        let mut num = vec![1];
        sort_colors(&mut num);
        assert_eq!(num, vec![1]);
    }

    #[test]
    fn t5() {
        let mut nums = vec![1, 2, 2, 2, 2, 0, 0, 0, 1, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 2]);
    }

    #[test]
    fn t6() {
        let mut nums = vec![
            2, 2, 1, 2, 1, 1, 1, 0, 0, 2, 1, 0, 2, 1, 2, 2, 1, 1, 1, 1, 1, 0, 2, 0, 2, 0, 2, 2, 1,
            0, 2, 1, 0, 2, 1, 2, 0, 0, 0, 0, 2, 1, 1, 2, 0, 1, 2, 2, 0, 0, 2, 2, 0, 1, 0, 1, 0, 0,
            1, 1, 1, 0, 0, 2, 2, 2, 1, 0, 0, 2, 1, 0, 1, 0, 2, 2, 1, 2, 1, 1, 2, 1, 1, 0, 0, 2, 1,
            0, 0,
        ];
        sort_colors(&mut nums);
        assert_eq!(
            nums,
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2,
            ]
        )
    }
}

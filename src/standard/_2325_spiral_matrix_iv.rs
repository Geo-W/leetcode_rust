use crate::utils::linked_list::ListNode;

pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![-1; n as usize]; m as usize];
    let mut head = head;

    let (mut left, mut right, mut top, mut bottom) = (0, n as usize - 1, 0, m as usize - 1);

    let mut cnt = 0;

    while cnt < m * n {
        // if true{
        for x in left..=right {
            if let Some(v) = head {
                ret[top][x] = v.val;
                head = v.next;
            } else {
                return ret;
            }
            cnt += 1;
        }
        top += 1;
        for y in top..=bottom {
            if let Some(v) = head {
                ret[y][right] = v.val;
                head = v.next;
            } else {
                return ret;
            }
            cnt += 1;
        }
        right -= 1;
        for x in (left..=right).rev() {
            if let Some(v) = head {
                ret[bottom][x] = v.val;
                head = v.next;
            } else {
                return ret;
            }
            cnt += 1;
        }
        bottom -= 1;
        for y in (top..=bottom).rev() {
            if let Some(v) = head {
                ret[y][left] = v.val;
                head = v.next;
            } else {
                return ret;
            }
            cnt += 1;
        }
        left += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::linked_list_from_vec;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            spiral_matrix(
                3,
                5,
                linked_list_from_vec(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]),
            ),
            vec_vec![[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]]
        );
    }
}

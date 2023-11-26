use std::cmp::min;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut ret = 0;

    let mut left = 0;
    let mut right = 0;

    let mut ptr = 0;

    let mut going_down = false;

    for p in 0..height.len()-1 {
        if height[p] != 0 {
            left = p;
            going_down = true;
            break;
        }
    }

    while ptr < height.len() - 2 {
        if height[ptr] > height[ptr + 1] {
            if going_down == false {
                going_down = true;
                ret += calculate_between(left, ptr, &height);
                dbg!(left);
                dbg!(ptr);
                dbg!(calculate_between(left, ptr, &height));
                left = ptr;
            }
        } else if height[ptr] < height[ptr + 1] {
            going_down = false;
        }
        ptr += 1;
    }

    ret
}

fn calculate_between(left: usize, right: usize, v: &Vec<i32>) -> i32 {
    let lower = min(v[left], v[right]);
    let mut ret = 0;
    for i in (left + 1)..right {
        ret += lower - v[i];
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subfn() {
        assert_eq!(
            calculate_between(3, 7, &vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            4
        );
        assert_eq!(calculate_between(0, 0, &vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn a() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn b() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}

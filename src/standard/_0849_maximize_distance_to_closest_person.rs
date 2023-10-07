pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut cur_start = 0;
    let mut cur_end = 0;
    let mut cal = false;

    for (idx, item) in seats.iter().chain([&1]).enumerate() {
        match item {
            0 => {
                if cal == false {
                    cal = true;
                    cur_start = idx;
                    cur_end = idx;
                } else {
                    cur_end = idx;
                }
            }
            _ => {
                if cal == true {
                    cal = false;
                    let new_max = if cur_start == 0 || cur_end == seats.len() - 1 {
                        cur_end - cur_start + 1
                    } else {
                        if (cur_end - cur_start) % 2 == 0 {
                            (cur_end - cur_start + 1) / 2 + 1
                        } else {
                            (cur_end - cur_start + 1) / 2
                        }
                    };
                    if new_max > max {
                        max = new_max;
                    }
                }
            }
        }
    }

    max as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    }

    #[test]
    fn a() {
        assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0]), 3);
    }

    #[test]
    fn b() {
        assert_eq!(max_dist_to_closest(vec![0, 1]), 1);
    }

    #[test]
    fn c() {
        assert_eq!(max_dist_to_closest(vec![1, 0, 0, 1]), 1);
    }
}

pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    fn calc(str: std::iter::Skip<std::str::Chars>, pos: &mut Vec<i32>, n: i32) -> i32 {
        let mut cnt = 0;
        for char in str {
            match char {
                'L' => {
                    if pos[1] == 0 {
                        break;
                    } else {
                        pos[1] -= 1;
                    }
                }
                'R' => {
                    if pos[1] == n - 1 {
                        break;
                    } else {
                        pos[1] += 1;
                    }
                }
                'U' => {
                    if pos[0] == 0 {
                        break;
                    } else {
                        pos[0] -= 1;
                    }
                }
                'D' => {
                    if pos[0] == n - 1 {
                        break;
                    } else {
                        pos[0] += 1;
                    }
                }
                _ => unreachable!()
            }
            cnt +=1;
        }

        cnt
    }
    let mut tmp = start_pos.clone();

    (0..s.len()).map(|x| {
        let ret = calc(s.chars().skip(x), &mut tmp, n);
        tmp[0] = start_pos[0];
        tmp[1] = start_pos[1];
        ret
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            execute_instructions(3, vec![0, 1], "RRDDLU".to_string()),
            vec![1, 5, 4, 3, 1, 0]
        );
    }
}

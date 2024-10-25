use std::collections::VecDeque;

pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    if  (k as usize) < skills.len(){
        let mut cur_player = (0, skills[0]);
        let mut dq = VecDeque::from_iter(skills.into_iter().enumerate().skip(1));
        let mut win_cnt = 0;
        while win_cnt < k {
            let nxt_player = dq.pop_front().unwrap();
            if cur_player.1 > nxt_player.1 {
                win_cnt +=1;
                dq.push_back(nxt_player);
            } else {
                cur_player = nxt_player;
                win_cnt = 1;
                dq.push_back(cur_player);
            }
        }
        cur_player.0 as i32
    } else {
        skills.into_iter().enumerate().max_by_key(|x| x.1).unwrap().0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
    }

    #[test]
    fn b() {
        assert_eq!(find_winning_player(vec![2, 5, 4], 3), 1);
    }
}

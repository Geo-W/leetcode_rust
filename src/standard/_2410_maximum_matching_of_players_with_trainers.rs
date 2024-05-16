pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut players = players;
    players.sort_unstable();
    players.reverse();
    let mut trainers = trainers;
    trainers.sort();
    trainers.reverse();

    let mut ptr_p = 0;
    let mut ptr_t = 0;
    unsafe {
        while ptr_p <= players.len() - 1 && ptr_t <= trainers.len() - 1 {
            if players.get_unchecked(ptr_p) > trainers.get_unchecked(ptr_t) {
                ptr_p +=1;
            } else {
                ret +=1;
                ptr_p+=1;
                ptr_t+=1;
            }
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
            match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]),
            2
        );
    }
}

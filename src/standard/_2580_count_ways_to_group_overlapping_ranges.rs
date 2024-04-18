pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
    let mut vec = ranges;
    vec.sort_by_key(|i| i[0]);
    let mut ret = Vec::with_capacity(vec.len());
    ret.push(vec[0].clone());
    let mut res = 2;

    vec.into_iter().skip(1).for_each(|item| {
        if item[0] <= ret[ret.len() - 1][1] && item[1] >= ret[ret.len() - 1][1] {
            let new_item = vec![ret[ret.len() - 1][0], item[1]];
            ret.pop().unwrap();
            ret.push(new_item);
        } else if item[0] > ret[ret.len() - 1][1] {
            ret.push(item);
            res = (res * 2) % 1000000007
        }
    });
    // dbg!(ret);
    res
}

#[cfg(test)]
mod tests {
    use crate::vec_vec;
    use super::*;

    #[test]
    fn a() {
        assert_eq!(count_ways(vec_vec![[6,10],[5,15]]), 2);
    }
    
    #[test]
    fn b() {
        assert_eq!(count_ways(vec_vec![[1,3],[10,20],[2,5],[4,8]]), 4);
    }
    
    #[test]
    fn c() {
        assert_eq!(count_ways(vec_vec![[5,11],[20,22],[1,3],[21,22],[11,11]]), 8);
    }
}
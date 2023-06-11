fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result = Vec::new();

    for (i, num) in nums.iter().enumerate() {
        if !map.contains_key(&(target - num)) {
            map.insert(nums[i], i as i32);
            // println!("{:?}",map);
        } else {
            result.push(i as i32);
            result.push(map.get(&(target - num)).unwrap().clone());
            // println!("{target}, {num}");
        }
    }
    println!("{:?}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut result = two_sum(vec![2, 7, 11, 15], 9);
        result.sort();
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn b() {
        let mut result = two_sum(vec![3, 2, 4], 6);
        result.sort();
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn c() {
        let mut result = two_sum(vec![3, 3], 6);
        result.sort();
        assert_eq!(result, [0, 1]);
    }
}

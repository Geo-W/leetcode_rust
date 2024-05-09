pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let length = plants.len() / 2;

    fn watering(plants: &mut dyn Iterator<Item = &i32>, full_capacity: i32) -> (i32, i32) {
        let mut ret = 0;
        let mut capacity = full_capacity;
        for plant in plants {
            if capacity >= *plant {
            } else {
                capacity = full_capacity;
                ret += 1;
            }
            capacity -= *plant;
        }
        (ret, capacity)
    }
    let (ret_a, c_a) = watering(&mut plants[0..length].iter(), capacity_a);
    let (ret_b, c_b) = watering(&mut plants.iter().rev().take(length), capacity_b);
    let ret_c = if plants.len() % 2 != 0 && std::cmp::max(c_a, c_b) < plants[length] {
        1
    } else {
        0
    };

    ret_a + ret_b + ret_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
    }
}

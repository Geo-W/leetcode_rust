pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut c = capacity;
    plants
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (idx, plant)| {
            if c >= plant {
                acc += 1;
                c -= plant;
            } else {
                acc += idx * 2 + 1;
                c = capacity - plant;
            }
            acc
        }) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(watering_plants(vec![2, 2, 3, 3], 5), 14);
    }
}

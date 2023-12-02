pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut trips = trips;
    trips.sort_by_key(|x| (x[1], x[2]));
    let mut status = Status {
        customers: vec![],
        sum: 0,
    };
    for trip in trips {
        let mut ptr = status.customers.len();
        while ptr >= 1 {
            if status.customers[ptr - 1][2] <= trip[1] {
                status.sum -= status.customers[ptr - 1][0];
                status.customers.remove(ptr - 1);
            }
            ptr -= 1;
        }
        status.sum += trip[0];
        if status.sum > capacity {
            return false;
        }
        status.customers.push(trip);
    }

    true
}

struct Status {
    customers: Vec<Vec<i32>>,
    sum: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(car_pooling(vec_vec![[2, 1, 5], [3, 3, 7]], 4), false);
    }
}

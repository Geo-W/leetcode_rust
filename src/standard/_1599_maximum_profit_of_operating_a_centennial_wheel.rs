pub fn min_operations_max_profit(
    customers: Vec<i32>,
    boarding_cost: i32,
    running_cost: i32,
) -> i32 {
    let mut ret = -1;

    let mut cur_profit = 0;
    let mut max_profit = 0;
    let mut round = 0;
    let mut cur_customer = 0;

    for i in customers {
        cur_customer += i;
        add_profit(
            boarding_cost,
            running_cost,
            &mut ret,
            &mut cur_profit,
            &mut max_profit,
            &mut round,
            &mut cur_customer,
        );
    }
    while cur_customer > 0 {
        add_profit(
            boarding_cost,
            running_cost,
            &mut ret,
            &mut cur_profit,
            &mut max_profit,
            &mut round,
            &mut cur_customer,
        );
    }

    ret
}

fn add_profit(
    boarding_cost: i32,
    running_cost: i32,
    ret: &mut i32,
    cur_profit: &mut i32,
    max_profit: &mut i32,
    round: &mut i32,
    cur_customer: &mut i32,
) {
    *round += 1;
    *cur_profit -= running_cost;
    if *cur_customer >= 4 {
        *cur_profit += 4 * boarding_cost;
        *cur_customer -= 4;
    } else {
        *cur_profit += *cur_customer * boarding_cost;
        *cur_customer -= *cur_customer;
    }
    if cur_profit > max_profit {
        *max_profit = *cur_profit;
        *ret = *round;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(min_operations_max_profit(vec![8, 3], 5, 6), 3);
    }

    #[test]
    fn b() {
        assert_eq!(min_operations_max_profit(vec![10, 10, 6, 4, 7], 3, 8), 9);
    }
}

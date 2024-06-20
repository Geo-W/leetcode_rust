pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    100 - if purchase_amount % 10 < 5 {
        purchase_amount / 10 * 10
    } else {
        (purchase_amount / 10 + 1) * 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(account_balance_after_purchase(9), 90);
    }

    #[test]
    fn b() {
        assert_eq!(account_balance_after_purchase(15), 80);
    }
}

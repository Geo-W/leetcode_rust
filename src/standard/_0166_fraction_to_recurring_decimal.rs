pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let mut map = std::collections::HashMap::new();
    let mut ret = String::new();
    let mut decimal = String::new();
    let mut repeated = false;
    let mut decimal_idx = 0;

    if !(numerator > 0 && denominator > 0 || numerator < 0 && denominator < 0) {
        ret.push('-');
    }
    let mut numerator = (numerator as i64).abs();
    let denominator = (denominator as i64).abs();
    if numerator == 0 {
        return "0".to_string();
    }
    if numerator.abs() > denominator.abs() {
        ret.push_str(&(numerator / denominator).to_string());
        if numerator % denominator == 0 {
            return ret;
        }
        ret.push('.');
        numerator = numerator % denominator;
    } else {
        ret.push_str("0.");
    };

    loop {
        if map.contains_key(&numerator) {
            decimal_idx = *map.get(&numerator).unwrap();
            repeated = true;
            break;
        }
        map.insert(numerator, decimal.len());
        numerator *= 10;
        while numerator < denominator {
            numerator *= 10;
            decimal.push('0');
            map.insert(0, decimal.len());
        }
        decimal.push_str(&(numerator / denominator).to_string());
        numerator = numerator % denominator;
        if numerator == 0 {
            break;
        }
    }
    if !repeated {
        ret.push_str(&decimal);
    } else {
        ret.push_str(&decimal[0..decimal_idx]);
        ret.push('(');
        ret.push_str(&decimal[decimal_idx..decimal.len()]);
        ret.push(')');
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(fraction_to_decimal(1, 2), 0.5.to_string());
    }

    #[test]
    fn b() {
        assert_eq!(fraction_to_decimal(2, 1), 2.to_string());
    }

    #[test]
    fn c() {
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)".to_string());
    }

    #[test]
    fn d() {
        assert_eq!(fraction_to_decimal(5, 5168), "0.0009(674922600619195046439628482972136222910216718266253869969040247678018575851393188854489164086687306501547987616099071207430340557275541795665634)".to_string());
    }

    #[test]
    fn e() {
        assert_eq!(fraction_to_decimal(0, 3), "0".to_string());
    }

    #[test]
    fn f() {
        assert_eq!(fraction_to_decimal(-50, 8), "-6.25".to_string());
    }

    #[test]
    fn g() {
        assert_eq!(
            fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125".to_string()
        );
    }
}

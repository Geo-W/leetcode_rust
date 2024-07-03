use std::collections::HashMap;

pub fn fraction_to_decimal(mut numerator: i32, mut denominator: i32) -> String {
    let mut map = HashMap::new();
    let mut ret = String::new();
    let mut decimal = String::new();
    let mut repeated = false;
    let mut idx = 0;

    if numerator > denominator {
        ret.push_str(&(numerator % denominator).to_string());
        if numerator % denominator == 0 {
            return ret;
        }
        ret.push('.');
        numerator = numerator % denominator;
    } else {
        ret.push_str("0.");
    }

    loop {
        dbg!(numerator);
        numerator *= 10;
        while numerator < denominator {
            numerator *= 10;
            if map.contains_key(&(numerator % denominator)) {
                println!("got 2");
                repeated = true;
                break;
            }
            decimal.push('0');
            map.insert(0, decimal.len());
            idx += 1;
        }
        if map.contains_key(&(numerator % denominator)) {
            repeated = true;
            decimal.push_str(&(numerator / denominator).to_string());
            println!("--- got at idx {}, value:{:?}", numerator % denominator, map.get(&(numerator % denominator)));
            break;
        }
        decimal.push_str(&(numerator / denominator).to_string());
        map.insert(numerator % denominator, decimal.len());
        idx += 1;
        numerator = numerator % denominator;
        if numerator == 0 {
            break;
        }
    }
    if !repeated {
        ret.push_str(&decimal);
    } else {
        ret.push('(');
        ret.push_str(&decimal);
        ret.push(')');
    }

    dbg!(map, decimal, repeated, &ret);

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
}

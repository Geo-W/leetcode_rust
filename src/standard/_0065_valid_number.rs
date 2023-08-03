/// A valid number can be split up into these components (in order):
///
///     A decimal number or an integer.
///     (Optional) An 'e' or 'E', followed by an integer.
///
/// A decimal number can be split up into these components (in order):
///
///     (Optional) A sign character (either '+' or '-').
///     One of the following formats:
///         One or more digits, followed by a dot '.'.
///         One or more digits, followed by a dot '.', followed by one or more digits.
///         A dot '.', followed by one or more digits.
///
/// An integer can be split up into these components (in order):
///
///     (Optional) A sign character (either '+' or '-').
///     One or more digits.
pub fn is_number(s: String) -> bool {
    let mut last_element= '0';
    let mut array = [0,0,0,0];
    for (index, char) in s.chars().enumerate() {
        match char {
            '+' | '-' => {
                if array[1] == 0 && index != 0 {
                    return false;
                } else if array[1] == 1 && last_element != 'e' && last_element != 'E' {
                    return false
                } else if array[3] == 1 {
                    return false;
                }
                array[3] = 1;
            }
            '0'..='9' => {
                array[2] = 1;
            }
            '.' => {
                if array[1] == 1 {
                    return false;
                } else if array[0] == 1 {
                    return false;
                }
                array[0] = 1;
            }
            'e' | 'E' => {
                array[3] = 0;
                if array[2] == 0 {
                    return false;
                } else if array[1] == 1{
                    return false;
                }
                array[1] = 1;
                array[2] = 0;
            }
            _ => {
                return false;
            }
        }
        last_element = char;
    }
    return if array[2] == 1 {
        true
    } else {
        false
    };
}

pub fn is_number2(s: String) -> bool {
    let mut dot = false;
    let mut expo = false;
    let mut number = false;
    let mut operator = false;
    let mut last_element= '0';
    for (index, char) in s.chars().enumerate() {
        match char {
            '+' | '-' => {
                if expo == false && index != 0 {
                    return false;
                } else if expo == true && last_element != 'e' && last_element != 'E' {
                    return false
                } else if operator == true {
                    return false;
                }
                operator = true;
            }
            '0'..='9' => {
                number = true;
            }
            '.' => {
                if expo {
                    return false;
                } else if dot == true {
                    return false;
                }
                dot = true;
            }
            'e' | 'E' => {
                operator = false;
                if number == false {
                    return false;
                } else if expo == true{
                    return false;
                }
                expo = true;
                number = false;
            }
            _ => {
                return false;
            }
        }
        last_element = char;
    }
    return if number {
        true
    } else {
        false
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_number("2".to_string()), true);
    }

    #[test]
    fn a() {
        assert_eq!(is_number("002".to_string()), true);
    }

    #[test]
    fn b() {
        assert_eq!(is_number("-0.2".to_string()), true);
    }

    #[test]
    fn c() {
        assert_eq!(is_number("+3.2".to_string()), true);
    }

    #[test]
    fn d() {
        assert_eq!(is_number("4.".to_string()), true);
    }

    #[test]
    fn e() {
        assert_eq!(is_number("-.9".to_string()), true);
    }

    #[test]
    fn f() {
        assert_eq!(is_number("2e10".to_string()), true);
    }

    #[test]
    fn g() {
        assert_eq!(is_number("e".to_string()), false);
    }

    #[test]
    fn h() {
        assert_eq!(is_number("e3".to_string()), false);
    }

    #[test]
    fn i() {
        assert_eq!(is_number("53.5e93".to_string()), true);
    }

    #[test]
    fn j() {
        assert_eq!(is_number("3e+7".to_string()), true);
    }

    #[test]
    fn k() {
        assert_eq!(is_number("+6e-1".to_string()), true);
    }

    #[test]
    fn l() {
        assert_eq!(is_number("99e2.5".to_string()), false);
    }

    #[test]
    fn m() {
        assert_eq!(is_number("6.5.5".to_string()), false);
    }

        #[test]
    fn n() {
        assert_eq!(is_number("459277e38+".to_string()), false);
    }
}

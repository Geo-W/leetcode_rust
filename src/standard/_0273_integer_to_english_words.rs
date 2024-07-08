pub fn number_to_words(num: i32) -> String {
    fn three_digit_to_words(mut num: i32) -> String {
        let mut ret = String::new();
        let num_to_str = |x| match x {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            10 => "Ten",
            11 => "Eleven",
            12 => "Twelve",
            13 => "Thirteen",
            14 => "Fourteen",
            15 => "Fifteen",
            16 => "Sixteen",
            17 => "Seventeen",
            18 => "Eighteen",
            19 => "Nineteen",
            20 => "Twenty",
            30 => "Thirty",
            40 => "Forty",
            50 => "Fifty",
            60 => "Sixty",
            70 => "Seventy",
            80 => "Eighty",
            90 => "Ninety",
            _ => "",
        };

        match num / 100 {
            1..=9 => {
                ret.push_str(&num_to_str(num / 100));
                ret.push_str(" Hundred")
            }
            _ => {}
        }
        num %= 100;

        match num / 10 {
            2..=9 => {
                if !ret.is_empty() {
                    ret.push(' ');
                }
                ret.push_str(&num_to_str(num / 10 * 10));
                num %= 10;
            }
            _ => {}
        }
        if !ret.is_empty() && num != 0 {
            ret.push(' ');
        }
        ret.push_str(&num_to_str(num));
        ret
    }
    let mut ret = vec![];
    let mut num = num;
    let post_fix = ["", " Thousand", " Million", " Billion", " Trillion"];
    let mut post_fix_idx = 0;
    while num > 0 {
        let mut s = String::new();
        let process_num = num % 1000;
        num /= 1000;
        if process_num != 0 {
            s.push_str(&three_digit_to_words(process_num));
            s.push_str(post_fix[post_fix_idx]);
            ret.push(s);
        }
        post_fix_idx += 1;
    }

    ret.reverse();
    ret.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(number_to_words(123), "One Hundred Twenty Three".to_string());
    }

    #[test]
    fn b() {
        assert_eq!(
            number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
    }

    #[test]
    fn d() {
        assert_eq!(number_to_words(1000010), "One Million Ten".to_string());
    }

    #[test]
    fn e() {
        assert_eq!(
            number_to_words(123000),
            "One Hundred Twenty Three Thousand".to_string()
        );
    }
}

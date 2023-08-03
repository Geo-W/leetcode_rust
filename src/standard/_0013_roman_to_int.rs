pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut result = 0;
    let bt = s.as_bytes();
    let len = bt.len();
    let mut i:usize = 0;
    while i<len{
        match bt[i] {
            73 =>{
                match bt.get(i+1).unwrap_or(&0){
                    86 => {
                        result +=4;
                        i +=2;
                    }
                    88=>{
                        result +=9;
                        i +=2;
                    }
                    _=>{result += 1;i+=1;}
                }
            }
            86 =>{result += 5; i+=1}
            88 =>{
                match bt.get(i+1).unwrap_or(&0){
                    76 =>{
                        result +=40;
                        i += 2;
                    }
                    67 =>{
                        result +=90;
                        i+=2;
                    }
                    _=>{result +=10; i+=1;}
                }
            }
            76 =>{result +=50;i+=1}
            67 =>{
                match bt.get(i+1).unwrap_or(&0){
                    68 =>{
                        result += 400;
                        i+=2;
                    }
                    77 =>{
                        result +=900;
                        i+=2;
                    }
                    _  =>{ result+=100; i+=1}
                }
            }
            68 =>{result +=500;i+=1}
            77 => {result +=1000; i+=1}
            _ =>{}
        }
    }
    result as i32
}



pub fn roman_to_int2(s: String) -> i32 {
    s.chars().fold((0, ' '), |res, ch| {
        match (res.1, ch) {
            ('I', 'V') => (res.0 + 3, 'V'),
            ('I', 'X') => (res.0 + 8, 'X'),
            ('X', 'L') => (res.0 + 30, 'L'),
            ('X', 'C') => (res.0 + 80, 'C'),
            ('C', 'D') => (res.0 + 300, 'D'),
            ('C', 'M') => (res.0 + 800, 'M'),
            (_, 'I') => (res.0 + 1, 'I'),
            (_, 'V') => (res.0 + 5, 'V'),
            (_, 'X') => (res.0 + 10, 'X'),
            (_, 'L') => (res.0 + 50, 'L'),
            (_, 'C') => (res.0 + 100, 'C'),
            (_, 'D') => (res.0 + 500, 'D'),
            (_, 'M') => (res.0 + 1000, 'M'),
            (_, _) => unreachable!(),
        }
    }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn b() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn c() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}

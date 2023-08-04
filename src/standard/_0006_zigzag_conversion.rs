pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        s
    } else {
        let mut vec = vec![String::new();num_rows as usize];
        let mut it = Cycler::new(num_rows as usize);
        for c in s.chars() {
            vec[it.next().unwrap() - 1].push(c);
        }
        vec.join("").to_string()
    }
}

struct Cycler {
    max_value: usize,
    current_value: usize,
    increasing: bool,
}

impl Cycler {
    fn new(max_value: usize) -> Self {
        Cycler {
            max_value,
            current_value: 1,
            increasing: true,
        }
    }
}

impl Iterator for Cycler {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value == self.max_value {
            self.increasing = false;
        } else if self.current_value == 1 {
            self.increasing = true;
        }

        let value = self.current_value;
        if self.increasing {
            self.current_value += 1;
        } else {
            self.current_value -= 1;
        }

        Some(value)
    }
}


#[cfg(test)]
mod tests {
    use crate::standard::_0006_zigzag_conversion::convert;

    #[test]
    fn t1() {
        convert("PAYPALISHIRING".to_string(), 4);
    }

    #[test]
    fn t2() {
        convert("AB".to_string(), 1);
    }
}
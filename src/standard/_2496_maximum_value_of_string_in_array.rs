/// Get max value in this vector. The value of an alphanumeric string can be defined as:
///     The numeric representation of the string in base 10, if it comprises of digits only.
///     The length of the string, otherwise.
pub fn maximum_value(strs: Vec<String>) -> i32 {
    let mut max = 0;
    strs.iter().for_each(|s| {
        let val;
        match s.parse::<i32>() {
            Ok(v) => {
                val = v;
            }
            Err(_) => {
                val = s.len() as i32;
            }
        }
        if val > max {
            max = val;
        }
    });
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = maximum_value(["alic3", "bob", "3", "4", "00000"].iter().map(|i| i.to_string()).collect());
        assert_eq!(result, 5);
    }
}

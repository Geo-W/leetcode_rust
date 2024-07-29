pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack = vec![];
    for s in operations {
        match s.as_str() {
            "+" => {
                stack.push(
                    stack[stack.len() - 1] + stack[stack.len() - 2]
                );
            }
            "D" => {
                stack.push(stack.last().unwrap()*2);
            }
            "C" => {
                stack.pop();
            }
            _ => {
                stack.push(s.parse::<i32>().unwrap());
            }
        }
    }

    stack.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::vec_string;
    use super::*;

    #[test]
    fn a() {
        assert_eq!(cal_points(vec_string!["5", "2", "C", "D", "+"]), 30);
    }
}

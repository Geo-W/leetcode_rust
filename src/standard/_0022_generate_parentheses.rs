pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ret = vec![];
    let mut tmp = Item::default();

    fn dfs(ret: &mut Vec<String>, tmp: &mut Item, n: i32) {
        if tmp.left == n && tmp.right == n {
            ret.push(tmp.s.clone());
        }

        if tmp.left < n {
            tmp.s.push('(');
            tmp.left += 1;
            dfs(ret, tmp, n);
            tmp.s.pop();
            tmp.left -= 1;
        }

        if tmp.right < tmp.left {
            tmp.s.push(')');
            tmp.right += 1;
            dfs(ret, tmp, n);
            tmp.s.pop();
            tmp.right -= 1;
        }
    }
    dfs(&mut ret, &mut tmp, n);

    ret
}

#[derive(Default)]
struct Item {
    s: String,
    left: i32,
    right: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            generate_parenthesis(3),
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn b() {
        assert_eq!(generate_parenthesis(1), vec_string!["()"]);
    }
}

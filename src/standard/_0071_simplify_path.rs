pub fn simplify_path(path: String) -> String {
    let mut s = path.split('/').collect::<Vec<_>>();
    if s.last() == Some(&"") {
        s.pop();
    }
    let mut del = 0;
    for str in s.iter_mut().rev() {
        match str {
            &mut "." => {
                *str = "";
            }
            &mut ".." => {
                del += 1;
                *str = "";
            }
            &mut "" => {}
            _ => {
                if del > 0 {
                    del -= 1;
                    *str = "";
                }
            }
        }
    }
    let mut ret = String::new();
    for i in s {
        if !i.is_empty() {
            ret.push('/');
            ret.push_str(i);
        }
    }

    if ret.is_empty() {
        ret.push('/');
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            simplify_path("/a/./b//../../c/".to_string()),
            "/c".to_string()
        );
    }
}

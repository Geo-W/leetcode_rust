/// Remove C++ comments: string"//" denotes line comment, string "/*" and "*/" constitute block comment.
pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut long_comment = false;
    let mut ret = vec![];
    let mut tmp = "".to_string();
    for line in source {
        let len = line.len();
        let mut i: usize = 0;
        let mut t = if tmp.len() == 0 {
            String::new()
        } else {
            tmp
        };
        'whileloop: while i < len {
            if i + 1 < len {
                match long_comment {
                    false => {
                        if &line[i..=i + 1] == "/*" {
                            long_comment = true;
                            i += 1;
                        } else if &line[i..=i + 1] == "//" {
                            break 'whileloop;
                        } else {
                            t.push_str(&line[i..i + 1])
                        }
                    }
                    true => {
                        if &line[i..=i + 1] == "*/" {
                            long_comment = false;
                            i += 1;
                        }
                    }
                }
            } else {
                if !long_comment {
                    t.push_str(&line[i..=i])
                }
            }
            i += 1;
        }
        if long_comment == true {
            tmp = t;
        } else {
            check_push(&mut ret, t);
            tmp = String::new();
        }
    }
    ret
}

fn check_push(vec: &mut Vec<String>, s: String) {
    if s.as_str() != "" {
        vec.push(s);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn it_works() {
        let vec = vec_string!["/*Test program */", "int main()", "{ ", "  // variable declaration ", "int a, b, c;", "/* This is a test", "   multiline  ", "   comment for ", "   testing */", "a = b + c;", "}"];
        let result = remove_comments(vec);
        assert_eq!(result, vec_string!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]);
    }

    #[test]
    fn t2() {
        let vec = vec_string!["a/*comment", "line", "more_comment*/b"];
        assert_eq!(remove_comments(vec), vec_string!["ab"])
    }

    #[test]
    fn t3() {
        let v = vec_string!["struct Node{", "    /*/ declare members;/**/", "    int size;", "    /**/int val;", "};"];
        assert_eq!(remove_comments(v),
                   vec_string!["struct Node{", "    ", "    int size;", "    int val;", "};"]
        )
    }

    #[test]
    fn t4() {
        let v = vec_string!["a//*b//*c", "blank", "d/*/e*//f"];
        assert_eq!(remove_comments(v),
                   vec_string!["a", "blank", "d/f"])
    }

    #[test]
    fn t5() {
        let v = vec_string!["a/*/b//*c", "blank", "d//*e/*/f"];

        assert_eq!(remove_comments(v),
                   vec_string!["af"])
    }

    #[test]
    fn t6() {
        let v = vec_string!["a/*/b//*c","blank","d/*/e*//f"];
        assert_eq!(remove_comments(v), vec_string!["ae*"])
    }
}

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut ret = vec![];
    let mut cur = vec![];
    let max_width = max_width as usize;
    let mut remain_length = max_width;
    for word in words {
        recur(&mut ret, &mut cur, max_width, &mut remain_length, word);
    }
    ret.push(normal_align(&mut cur, max_width));
    ret
}

#[inline]
fn normal_align(cur: &mut Vec<String>, max_width: usize) -> String {
    let mut tmp = cur.join(" ");
    tmp.extend(std::iter::repeat(" ").take(max_width - tmp.len()));
    tmp
}

#[inline]
fn empty_align(cur: &mut Vec<String>, max_width: usize) -> String {
    let empty_length = max_width - cur.iter().map(|x| x.len()).sum::<usize>();
    let empty_spaces = cur.len() - 1;
    let empty = if empty_length % empty_spaces == 0 {
        vec![empty_length / empty_spaces; empty_spaces]
    } else {
        let mut tmp = vec![empty_length / empty_spaces; empty_spaces];
        let remain = empty_length % empty_spaces;
        for i in 0..remain {
            tmp[i] += 1;
        }
        tmp
    };
    cur.iter_mut().enumerate().for_each(|(idx, i)| {
        if idx < empty.len() {
            i.extend(std::iter::repeat(" ").take(empty[idx]))
        }
    });
    cur.join("")
}

#[inline]
fn recur(
    ret: &mut Vec<String>,
    cur: &mut Vec<String>,
    max_width: usize,
    remain_length: &mut usize,
    word: String,
) {
    let w_len = word.len();
    if w_len < *remain_length {
        cur.push(word);
        *remain_length -= w_len;
        *remain_length -= 1;
    } else if w_len == *remain_length {
        cur.push(word);
        *remain_length -= w_len;
    } else {
        ret.push(if cur.len() > 1 {
            empty_align(cur, max_width)
        } else {
            normal_align(cur, max_width)
        });
        *remain_length = max_width;
        cur.clear();
        recur(ret, cur, max_width, remain_length, word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            full_justify(
                vec_string![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            vec_string!["This    is    an", "example  of text", "justification.  "]
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            full_justify(
                vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
                16
            ),
            vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            full_justify(
                vec_string![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            vec_string![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}

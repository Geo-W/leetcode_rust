pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut m_acc = 0;
    let mut p_acc = 0;
    let mut g_acc = 0;
    let mut ret = 0;

    for (idx, item) in garbage.into_iter().enumerate() {
        let mut m = 0;
        let mut p = 0;
        let mut g = 0;
        for c in item.chars() {
            match c {
                'G' => g += 1,
                'P' => p += 1,
                'M' => m += 1,
                _ => unimplemented!(),
            }
        }
        if m > 0 {
            ret += m + m_acc;
            m_acc = 0;
        }
        if p > 0 {
            ret += p + p_acc;
            p_acc = 0;
        }
        if g > 0 {
            ret += g + g_acc;
            g_acc = 0;
        }

        m_acc += *travel.get(idx).unwrap_or(&0);
        g_acc += *travel.get(idx).unwrap_or(&0);
        p_acc += *travel.get(idx).unwrap_or(&0);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn a() {
        assert_eq!(
            garbage_collection(vec_string!["G", "P", "GP", "GG"], vec![2, 4, 3]),
            21
        );
    }
}

pub fn entity_parser(text: String) -> String {
    text.replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&frasl;", "/")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
            "& is an HTML entity but &ambassador; is not.".to_string()
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            entity_parser("and I quote: &quot;...&quot;".to_string()),
            "and I quote: \"...\"".to_string()
        );
    }

    #[test]
    fn c() {
        assert_eq!(
            entity_parser("Stay home! Practice on Leetcode :)".to_string()),
            "Stay home! Practice on Leetcode :)".to_string()
        );
    }

    #[test]
    fn d() {
        assert_eq!(
            entity_parser("x &gt; y &amp;&amp; x &lt; y is always false".to_string()),
            "x > y && x < y is always false".to_string()
        );
    }

    #[test]
    fn e() {
        assert_eq!(
            entity_parser("leetcode.com&frasl;problemset&frasl;all".to_string()),
            "leetcode.com/problemset/all".to_string()
        );
    }

    #[test]
    fn f() {
        assert_eq!(entity_parser("&amp;gt;".to_string()), "&gt;".to_string());
    }
}

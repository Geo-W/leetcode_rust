/// Given an input string s, reverse the order of the words.
/// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
/// Return a string of the words in reverse order concatenated by a single space.
pub fn reverse_words(s: String) -> String {
    // s.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>().into_iter().rev().reduce(|cur, next| format!("{} {}", cur, &next)).unwrap()
    s.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>().into_iter().rev().collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;


}

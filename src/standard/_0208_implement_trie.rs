#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    stop: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for byte in word.as_bytes() {
            let idx = *byte as usize - 97;
            if !node.children[idx].is_some() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.stop = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for byte in word.as_bytes() {
            let idx = *byte as usize - 97;
            if let Some(v) = &node.children[idx] {
                node = v;
            } else {
                return false;
            }
        }
        if !node.stop {
            return false;
        }
        true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for byte in prefix.as_bytes() {
            let idx = *byte as usize - 97;
            if let Some(v) = &node.children[idx] {
                node = v;
            } else {
                return false;
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(true, true);
    }
}

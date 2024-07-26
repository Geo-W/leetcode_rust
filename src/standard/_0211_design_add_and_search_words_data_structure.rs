use std::collections::VecDeque;

#[derive(Default)]
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    stop: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for byte in word.as_bytes() {
            let idx = *byte as usize - 97;
            if !node.children[idx].is_some() {
                node.children[idx] = Some(Box::new(WordDictionary::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.stop = true;
    }

    fn search(&self, word: String) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        for byte in word.as_bytes() {
            if *byte == 46 {
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    for child in &node.children {
                        if let Some(v) = child.as_ref() {
                            queue.push_back(v);
                        }
                    }
                }
            } else {
                let idx = *byte as usize - 97;
                let mut found = false;
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    if let Some(v) = &node.children[idx] {
                        found = true;
                        queue.push_back(v);
                    }
                }
                if !found {
                    return false;
                }
            }
        }

        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            if node.stop {
                return true;
            }
        }
        false
    }
}

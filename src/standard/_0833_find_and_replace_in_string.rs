use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Index;

/// for index in indices, if s[index] == sources[index] then replace it with targets[index]
pub fn find_replace_string(mut s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
    let mut heap = BinaryHeap::new();

    for ((idx, ss), ts) in indices.into_iter().zip(sources.into_iter()).zip(targets.into_iter()) {
        heap.push(Idx {
            index: idx as usize,
            source: ss,
            targets: ts,
        })
    };

    while let Some(v) = heap.pop() {
        if let Some(substr) =  s.get(v.index..(v.index+v.source.len())) {
            if substr == v.source {
                s.replace_range(v.index..(v.index + v.source.len()), &v.targets);
            }
        }
    }
    s
}

#[derive(Debug)]
struct Idx {
    index: usize,
    source: String,
    targets: String,
}

impl Eq for Idx {}

impl PartialEq<Self> for Idx {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl PartialOrd<Self> for Idx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for Idx {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_replace_string("abcd".to_string(), vec![0, 2], vec!["a".to_string(), "cd".to_string()],
                                       vec!["eee".to_string(), "ffff".to_string()]), "eeebffff".to_string());
    }
}

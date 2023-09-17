use std::collections::HashMap;
use rand;
use rand::Rng;

#[derive(Debug)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}


impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            map: Default::default(),
            vec: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        return if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.vec.len());
            self.vec.push(val);
            true
        };
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            None => {
                false
            }
            Some(v) => {
                let vec_len = self.vec.len();
                self.vec.swap(v, vec_len - 1);
                self.vec.pop().unwrap();
                if self.vec.len() > v {
                    self.map.insert(
                        self.vec[v],
                        v,
                    );
                }
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        self.vec[rand::thread_rng().gen_range(0, self.vec.len())]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut obj = RandomizedSet::new();
        let ret_1: bool = obj.insert(1);
        let ret_2: bool = obj.remove(1);
        let ret_1: bool = obj.insert(2);
        let ret_3: i32 = obj.get_random();
        assert_eq!(ret_3, 2);
    }

    #[test]
    fn b() {
        let mut obj = RandomizedSet::new();
        let r1 = obj.insert(0);
        let r2 = obj.insert(1);
        let r3 = obj.remove(0);
        let r4 = obj.insert(2);
        let r5 = obj.remove(1);
        let r6 = obj.get_random();
        assert_eq!((r1, r2, r3, r4, r5, r6), (true, true, true, true, true, 2));
    }

    #[test]
    fn c() {
        let mut obj = RandomizedSet::new();
        let r1 = obj.insert(3);
        let r2 = obj.insert(3);
        let r3 = obj.get_random();
        let r4 = obj.insert(1);
        let r5 = obj.remove(3);
        let r6 = obj.get_random();
        let r7 = obj.insert(0);
        let r8 = obj.remove(0);
        assert_eq!((r1, r2, r3, r4, r5, r6, r7, r8),
                   (true, false, 3, true, true, 1, true, true));
    }
}
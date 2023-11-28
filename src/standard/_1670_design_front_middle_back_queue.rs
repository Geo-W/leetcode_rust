use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            front: Default::default(),
            back: Default::default(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.front.push_front(val);
        if self.front.len() > self.back.len() + 1 {
            self.back.push_front(self.front.pop_back().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap())
        }
        self.front.push_back(val);
    }

    fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
        if self.back.len() > self.front.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }
    }

    fn pop_front(&mut self) -> i32 {
        if let Some(v) = self.front.pop_front() {
            if self.front.len() < self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            v
        } else {
            -1
        }
    }

    fn pop_middle(&mut self) -> i32 {
        if let Some(v) = self.front.pop_back() {
            if self.front.len() < self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            v
        } else {
            -1
        }
    }

    fn pop_back(&mut self) -> i32 {
        if let Some(v) = self.back.pop_back() {
            if self.front.len() > self.back.len() + 1 {
                self.back.push_front(self.front.pop_back().unwrap());
            }
            v
        } else {
            if let Some(v2) = self.front.pop_back() {
                v2
            } else {
                -1
            }
        }
    }

    fn v(&self) -> Vec<i32> {
        dbg!(&self.front);
        dbg!(&self.back);
        self.front
            .iter()
            .chain(self.back.iter())
            .map(|x| *x)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1);
        q.push_back(2);
        dbg!(q.v());
        q.push_middle(3);
        assert_eq!(q.v(), vec![1, 3, 2]);
        q.push_middle(4);
        assert_eq!(q.v(), vec![1, 4, 3, 2]);
        assert_eq!(q.pop_front(), 1);
        assert_eq!(q.v(), vec![4, 3, 2]);
        assert_eq!(q.pop_middle(), 3);
        assert_eq!(q.v(), vec![4, 2]);
        assert_eq!(q.pop_middle(), 4);
        assert_eq!(q.v(), vec![2]);
        assert_eq!(q.pop_back(), 2);
    }
}

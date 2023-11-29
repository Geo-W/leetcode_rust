use std::collections::VecDeque;

struct SmallestInfiniteSet {
    num: i32,
    extra_set: VecDeque<i32>
}


impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet{ num: 0, extra_set: Default::default() }
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.extra_set.is_empty() {
            self.num +=1;
            return self.num
        } else {
            self.extra_set.pop_front().unwrap()
        }
    }

    fn add_back(&mut self, num: i32) {
        if num <= self.num {
            match self.extra_set.binary_search(&num) {
                Ok(_) => {}
                Err(v) => { self.extra_set.insert(v, num);}
            }
        }
    }
}

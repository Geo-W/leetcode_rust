/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
/// You must implement a solution with O(1) time complexity for each function.
pub struct MinStack {
    min_stack: Vec<i32>,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack { min_stack: Vec::new(), stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            Some(v) => {
                if *v >= val {
                    self.min_stack.push(val)
                }
            }
            None => {
                self.min_stack.push(val)
            }
        }
    }

    pub fn pop(&mut self) {
        let a = self.stack.pop();
        match a {
            Some(v) => {
                if *self.min_stack.last().unwrap() == v {
                    self.min_stack.pop();
                }
            }
            _ => {}
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = MinStack::new();
        obj.push(0);
        obj.push(1);
        obj.push(0);
        let m1 = obj.get_min();
        obj.pop();
        let m2 = obj.get_min();
        assert_eq!(m1, 0);
        assert_eq!(m2, 0);
    }

    #[test]
    fn a() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        let m1 = obj.get_min();
        obj.pop();
        let top = obj.top();
        let m2 = obj.get_min();
        assert_eq!(m1, -3);
        assert_eq!(m2, -2);
        assert_eq!(top, 0);
    }
}

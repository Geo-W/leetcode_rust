#[derive(Debug)]
struct MyCircularQueue {
    inner: Vec<i32>,
    start: usize,
    end: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            inner: vec![-1; k as usize],
            start: 0,
            end: 0,
            len: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.inner[self.end] = value;
        self.end += 1;
        self.end %= self.inner.len();
        self.len += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.inner[self.start] = -1;
        self.start += 1;
        self.start %= self.inner.len();
        self.len -= 1;
        true
    }

    fn front(&self) -> i32 {
        self.inner[self.start]
    }

    fn rear(&self) -> i32 {
        self.inner[(self.end + self.inner.len() - 1) % self.inner.len()]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut q = MyCircularQueue::new(3);
        assert_eq!(q.en_queue(1), true);
        assert_eq!(q.en_queue(2), true);
        assert_eq!(q.en_queue(3), true);
        assert_eq!(q.en_queue(4), false);
        assert_eq!(q.rear(), 3);
        assert_eq!(q.is_full(), true);
        assert_eq!(q.de_queue(), true);
        assert_eq!(q.en_queue(4), true);
        assert_eq!(q.rear(), 4);
    }

    #[test]
    fn b() {
        let mut q = MyCircularQueue::new(8);
        q.en_queue(3);
        q.en_queue(9);
        q.en_queue(5);
        q.en_queue(0);
        dbg!(&q);
        q.de_queue();
        dbg!(&q);
        q.de_queue();
        dbg!(&q);
        assert_eq!(q.rear(), 0);
    }

    #[test]
    fn c() {
        let mut q = MyCircularQueue::new(3);
        q.en_queue(7);
        q.de_queue();
    }
}

#[derive(Default, Debug)]
struct StockSpanner {
    mono_stack: Vec<usize>,
    stock_list: Vec<i32>,
    idx: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        self.stock_list.push(price);
        while let Some(v) = self.mono_stack.last() {
            let v = *v;
            if price >= self.stock_list[v] {
                self.mono_stack.pop().unwrap();
            } else {
                break;
            }
        }
        let last_idx = match self.mono_stack.last() {
            None => 0,
            Some(&v) => v + 1,
        };
        self.mono_stack.push(self.idx);
        self.idx += 1;
        (self.idx - last_idx) as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut s = StockSpanner::new();
        assert_eq!(s.next(100), 1);
        assert_eq!(s.next(80), 1);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(70), 2);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(75), 4);
        assert_eq!(s.next(85), 6);
    }

    #[test]
    fn b() {
        let mut s = StockSpanner::new();
        assert_eq!(s.next(31), 1);
        assert_eq!(s.next(41), 2);
        assert_eq!(s.next(48), 3);
        assert_eq!(s.next(59), 4);
        assert_eq!(s.next(79), 5);
    }
}

struct DataStream {
    value: i32,
    k: i32,
    equal: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self { value, k, equal: 0 }
    }

    fn consec(&mut self, num: i32) -> bool {
        if num == self.value {
            self.equal += 1;
            if self.equal >= self.k {
                return true;
            }
        } else {
            self.equal = 0;
        }
        false
    }
}

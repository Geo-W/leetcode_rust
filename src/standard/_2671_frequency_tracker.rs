/// Design a data structure that keeps track of the values in it and answers some queries regarding their frequencies.
// Implement the FrequencyTracker class.
///     FrequencyTracker(): Initializes the FrequencyTracker object with an empty array initially.
///     void add(int number): Adds number to the data structure.
///     void deleteOne(int number): Deletes one occurrence of number from the data structure. The data structure may not contain number, and in this case nothing is deleted.
///     bool hasFrequency(int frequency): Returns true if there is a number in the data structure that occurs frequency number of times, otherwise, it returns false.

use std::collections::HashMap;

#[derive(Debug)]
struct FrequencyTracker {
    frequencies: HashMap<i32, i32>,
    data: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            frequencies: Default::default(),
            data: Default::default(),
        }
    }

    fn add(&mut self, number: i32) {
        let old_freq = self.data.entry(number).or_insert(0);
        self.frequencies.entry(*old_freq).and_modify(|e| *e -= 1);
        *old_freq += 1;
        *self.frequencies.entry(*old_freq).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        let freq = self.data.entry(number).or_insert(0);
        self.frequencies.entry(*freq).and_modify(|e| *e -= 1);
        if *freq != 0 {
            *freq -= 1;
            *self.frequencies.entry(*freq).or_insert(0) +=1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.frequencies.get(&frequency).unwrap_or(&0) > &0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut f = FrequencyTracker::new();
        f.add(3);
        f.add(3);
        assert_eq!(f.has_frequency(2), true);
    }

    #[test]
    fn b() {
        let mut f = FrequencyTracker::new();
        f.add(5);
        f.add(4);
        f.delete_one(6);
        f.delete_one(4);
        f.delete_one(7);
        assert_eq!(f.has_frequency(1), true);
    }

    #[test]
    fn c() {
        let mut f = FrequencyTracker::new();
        assert_eq!(f.has_frequency(1), false);
        f.add(3);
        assert_eq!(f.has_frequency(1), true);
        f.add(6);
        f.add(2);
        f.add(6);
        f.delete_one(6);
        f.delete_one(6);
        assert_eq!(f.has_frequency(2), false);
        f.add(2);
        assert_eq!(f.has_frequency(2), true);
        assert_eq!(f.has_frequency(1), true);
    }
}

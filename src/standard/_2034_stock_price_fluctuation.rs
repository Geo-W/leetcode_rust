use std::cmp::max;
use std::collections::{BTreeMap, HashMap};

/// You are given a stream of records about a particular stock. Each record contains a timestamp and the corresponding price of the stock at that timestamp.
struct StockPrice {
    records: HashMap<i32, i32>,
    sorted_price: BTreeMap<i32, i32>,
    max_ts: i32,
}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            records: HashMap::new(),
            sorted_price: Default::default(),
            max_ts: 0,
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        match self.records.insert(timestamp, price) {
            None => {}
            Some(v) => {
                if let Some(count) = self.sorted_price.get_mut(&v) {
                    if *count == 1 {
                        self.sorted_price.remove(&v);
                    } else {
                        *count -= 1;
                    }
                }
            }
        }
        *self.sorted_price.entry(price).or_insert(0) += 1;

        self.max_ts = max(self.max_ts, timestamp);
    }

    fn current(&self) -> i32 {
        self.records[&self.max_ts]
    }

    fn maximum(&self) -> i32 {
        *self.sorted_price.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.sorted_price.iter().next().unwrap().0
    }
}

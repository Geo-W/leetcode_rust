struct SnapshotArray {
    arr: Vec<Vec<(i32, i32)>>,
    snapshot: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            arr: vec![vec![(0, 0)]; length as usize],
            snapshot: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if let Some(v) = self.arr.get_mut(index as usize).unwrap().last_mut() {
            if v.0 != self.snapshot {
                self.arr[index as usize].push((self.snapshot, val));
            } else {
                (*v).1 = val;
            }
        } else {
            unreachable!();
        }
    }

    fn snap(&mut self) -> i32 {
        let tmp = self.snapshot;
        self.snapshot += 1;
        tmp
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let target = self.arr[index as usize].binary_search_by(|x| x.0.cmp(&snap_id)).unwrap_or_else(|v| v - 1);
        self.arr[index as usize][target].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn a() {
        let mut arr = SnapshotArray::new(3);
        arr.set(0, 5);
        assert_eq!(arr.snap(), 0);
        arr.set(0, 6);
        assert_eq!(arr.get(0, 0), 5);
    }
}

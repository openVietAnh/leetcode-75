use std::collections::{BinaryHeap, HashSet};

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
struct SmallestInfiniteSet {
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            heap: BinaryHeap::from_iter(-1000..=-1),
            set: HashSet::from_iter(1..=1000),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let val = self.heap.pop().unwrap() * -1;
        self.set.remove(&val);
        val
    }

    fn add_back(&mut self, num: i32) {
        if !self.set.contains(&num) {
            self.set.insert(num);
            self.heap.push(-num);
        }
    }
}

// Prevent unused code warnings
pub fn test_smallest_infinite_set() {
    let mut set = SmallestInfiniteSet::new();
    set.pop_smallest();
    set.add_back(1);
}

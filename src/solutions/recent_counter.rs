use std::collections::VecDeque;

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while self.queue.front().unwrap() + 3000 < t {
            self.queue.pop_front();
        }
        self.queue.len().try_into().unwrap()
    }
}

// Prevent unused code warnings
pub fn test_recent_counter() {
    let mut counter = RecentCounter::new();
    counter.ping(3000);
}

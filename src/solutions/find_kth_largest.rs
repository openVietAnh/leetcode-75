use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    for item in nums {
        heap.push(item);
    }
    for _ in 0..k - 1 {
        heap.pop();
    }
    heap.pop().unwrap()
}

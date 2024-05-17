use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![], vec![]];
    let h1: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
    let h2: HashSet<i32> = HashSet::from_iter(nums2.into_iter());
    for item in &h1 {
        if !h2.contains(&item) {
            result[0].push(*item);
        }
    }
    for item in h2 {
        if !h1.contains(&item) {
            result[1].push(item);
        }
    }
    result
}

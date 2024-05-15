pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut prefix = vec![];
    let mut cur = 0;
    for item in &nums {
        prefix.push(cur);
        cur += item;
    }
    cur = 0;
    let mut suffix = vec![];
    for item in nums.clone().into_iter().rev() {
        suffix.push(cur);
        cur += item;
    }
    for i in 0..nums.len() {
        if prefix[i] == suffix[nums.len() - i] {
            return i as i32;
        }
    }
    -1
}

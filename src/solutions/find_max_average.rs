pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut result: i32 = nums[0..k as usize].into_iter().sum();
    let mut curr = result;
    for i in k as usize..nums.len() {
        curr += nums[i] - nums[i - k as usize];
        if curr > result {
            result = curr
        }
    }
    result as f64 / k as f64
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut dp: Vec<i32> = nums.clone();
    let n = nums.len();
    for i in 0..n {
        result = std::cmp::max(result, dp[i]);
        for j in i + 2..n {
            if dp[j] < dp[i] + nums[j] {
                dp[j] = dp[i] + nums[j];
                result = std::cmp::max(result, dp[j]);
            }
        }
    }
    result
}

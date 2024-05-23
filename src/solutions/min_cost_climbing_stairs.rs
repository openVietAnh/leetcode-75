pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![i32::MAX; cost.len()];
    (dp[0], dp[1]) = (0, 0);
    let mut result = i32::MAX;
    for i in 0..cost.len() {
        if i + 1 < cost.len() {
            if dp[i + 1] > dp[i] + cost[i] {
                dp[i + 1] = dp[i] + cost[i];
            }
        } else {
            result = std::cmp::min(result, dp[i] + cost[i]);
        }
        if i + 2 < cost.len() {
            if dp[i + 2] > dp[i] + cost[i] {
                dp[i + 2] = dp[i] + cost[i];
            }
        } else {
            result = std::cmp::min(result, dp[i] + cost[i]);
        }
    }
    result
}

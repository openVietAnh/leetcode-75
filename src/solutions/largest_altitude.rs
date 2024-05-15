pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut result = 0;
    for item in gain {
        cur += item;
        result = std::cmp::max(result, cur);
    }
    result
}

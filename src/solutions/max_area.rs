pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_value = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    while left < right {
        max_value = std::cmp::max(
            max_value,
            (right - left) as i32 * std::cmp::min(height[left], height[right]),
        );
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_value
}

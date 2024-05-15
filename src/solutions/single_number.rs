pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in nums {
        result = result ^ i;
    }
    return result;
}

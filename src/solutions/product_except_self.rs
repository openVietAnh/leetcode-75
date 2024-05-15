// [1, 2, 3, 4]
// prefix_prod: [1, 2, 6, 24]
// suffix_prod: [4, 12, 24, 24]
// required_result: [24, 12, 8, 6]

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_prod = vec![];
    let mut suffix_prod = vec![];
    let (mut running_prefix, mut running_suffix) = (1, 1);
    for i in 0..nums.len() {
        running_prefix *= nums[i];
        prefix_prod.push(running_prefix);
        running_suffix *= nums[nums.len() - i - 1];
        suffix_prod.push(running_suffix);
    }
    let mut result = vec![];
    for i in 0..nums.len() {
        if i == 0 {
            result.push(suffix_prod[nums.len() - 2]);
        } else if i == nums.len() - 1 {
            result.push(prefix_prod[nums.len() - 2]);
        } else {
            result.push(prefix_prod[i - 1] * suffix_prod[nums.len() - i - 2]);
        }
    }
    result
}

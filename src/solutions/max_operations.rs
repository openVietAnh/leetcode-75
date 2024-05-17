use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    for item in nums {
        if count.contains_key(&(k - item)) && *count.get(&(k - item)).unwrap() != 0 {
            result += 1;
            *count.get_mut(&(k - item)).unwrap() -= 1;
        } else {
            count.entry(item).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    result
}

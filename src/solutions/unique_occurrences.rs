use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut count = HashMap::new();
    for item in arr {
        count.entry(item).and_modify(|v| *v += 1).or_insert(1);
    }
    let value_set: HashSet<i32> = HashSet::from_iter(count.values().into_iter().map(|v| *v as i32));
    value_set.len() == count.values().len()
}

use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut count1: HashMap<char, i32> = HashMap::new();
    let mut count2: HashMap<char, i32> = HashMap::new();
    for c in word1.chars() {
        count1.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    for c in word2.chars() {
        if !count1.contains_key(&c) {
            return false;
        }
        count2.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut count3: HashMap<i32, i32> = HashMap::new();
    let mut count4: HashMap<i32, i32> = HashMap::new();
    for v in count1.values() {
        count3.entry(*v).and_modify(|v| *v += 1).or_insert(1);
    }
    for v in count2.values() {
        count4.entry(*v).and_modify(|v| *v += 1).or_insert(1);
    }
    for (key, value) in count3.into_iter() {
        if !count4.contains_key(&key) {
            return false;
        } else {
            if *count4.get(&key).unwrap() != value {
                return false;
            }
        }
    }
    true
}

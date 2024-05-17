use std::collections::HashSet;

pub fn max_vowels(s: String, k: i32) -> i32 {
    let vowels: HashSet<char> = HashSet::from_iter(String::from("ueoai").chars());
    let c: Vec<char> = s.chars().collect();
    let mut result = 0;
    for i in 0..k as usize {
        if vowels.contains(&c[i]) {
            result += 1;
        }
    }
    let mut current = result;
    for i in k as usize..c.len() {
        if vowels.contains(&c[i]) {
            current += 1;
        }
        if vowels.contains(&c[i - k as usize]) {
            current -= 1;
        }
        result = std::cmp::max(result, current);
    }
    result
}

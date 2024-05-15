use std::collections::HashSet;

pub fn reverse_vowels(s: String) -> String {
    let vowels = HashSet::from(['u', 'e', 'o', 'a', 'i', 'U', 'E', 'O', 'A', 'I']);
    let mut c: Vec<char> = s.chars().collect();

    let (mut l, mut r) = (0, c.len() - 1);
    loop {
        while l < c.len() && !vowels.contains(&c[l]) {
            l += 1;
        }
        while r >= 0 && r < c.len() && !vowels.contains(&c[r]) {
            r -= 1;
        }
        if l < r && r < c.len() {
            (c[l], c[r]) = (c[r], c[l]);
            l += 1;
            r -= 1;
        } else {
            break;
        }
    }
    c.into_iter().collect()
}

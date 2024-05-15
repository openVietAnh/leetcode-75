pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }
    let mut ind = 0;
    let st: Vec<char> = s.chars().collect();
    for c in t.chars() {
        if c == st[ind] {
            ind += 1;
            if ind == s.len() {
                return true;
            }
        }
    }
    ind == s.len()
}

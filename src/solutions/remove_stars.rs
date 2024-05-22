pub fn remove_stars(s: String) -> String {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if c != '*' {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    stack.into_iter().collect()
}

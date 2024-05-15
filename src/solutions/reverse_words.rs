pub fn reverse_words(s: String) -> String {
    s.trim()
        .split(' ')
        .filter(|st| st.len() > 0)
        .map(|st| st.to_string())
        .rev()
        .collect::<Vec<String>>()
        .join(" ")
}

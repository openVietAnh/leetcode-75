pub fn merge_alternately(word1: String, word2: String) -> String {
    let (w1, w2): (Vec<char>, Vec<char>) = (word1.chars().collect(), word2.chars().collect());
    let mut result = String::from("");
    let (mut ind1, mut ind2) = (0, 0);
    while ind1 < word1.len() && ind2 < word2.len() {
        result += &(w1[ind1].to_string() + &w2[ind2].to_string());
        ind1 += 1;
        ind2 += 1;
    }
    while ind1 < word1.len() {
        result += &w1[ind1].to_string();
        ind1 += 1;
    }
    while ind2 < word2.len() {
        result += &w2[ind2].to_string();
        ind2 += 1;
    }
    result
}

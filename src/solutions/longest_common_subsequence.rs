pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; text2.len()]; text1.len()];
    let (chars1, chars2): (Vec<char>, Vec<char>) =
        (text1.chars().collect(), text2.chars().collect());
    dp[0][0] = if chars1[0] == chars2[0] { 1 } else { 0 };
    let mut found = false;
    for i in 0..text1.len() {
        if chars1[i] == chars2[0] {
            found = true;
        }
        dp[i][0] = if found { 1 } else { 0 }
    }
    found = false;
    for i in 0..text2.len() {
        if chars2[i] == chars1[0] {
            found = true;
        }
        dp[0][i] = if found { 1 } else { 0 }
    }
    for i in 1..text1.len() {
        for j in 1..text2.len() {
            dp[i][j] = if chars1[i] == chars2[j] {
                dp[i - 1][j - 1] + 1
            } else {
                std::cmp::max(dp[i][j - 1], dp[i - 1][j])
            }
        }
    }
    dp[text1.len() - 1][text2.len() - 1]
}

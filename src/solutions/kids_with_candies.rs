pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.clone().into_iter().max().unwrap();
    candies
        .into_iter()
        .map(|v| v + extra_candies >= max)
        .collect()
}

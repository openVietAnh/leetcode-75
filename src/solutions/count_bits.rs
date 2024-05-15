pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..(n + 1) {
        result.push(i32::count_ones(i) as i32);
    }
    result
}

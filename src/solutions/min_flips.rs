pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let (mut result, mut ind) = (0, 0);
    while 1 << ind <= a || 1 << ind <= b || 1 << ind <= c {
        let bit_a = a & 1 << ind == 1 << ind;
        let bit_b = b & 1 << ind == 1 << ind;
        let bit_c = c & 1 << ind == 1 << ind;
        if bit_c {
            if !bit_a && !bit_b {
                result += 1;
            }
        } else {
            if bit_a {
                result += 1;
            }
            if bit_b {
                result += 1;
            }
        }
        ind += 1;
    }
    result
}

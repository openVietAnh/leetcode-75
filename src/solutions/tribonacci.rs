pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        let (mut x, mut y, mut z) = (0, 1, 1);
        for i in 3..(n + 1) {
            let result = x + y + z;
            x = y;
            y = z;
            z = result;
        }
        z
    }
}

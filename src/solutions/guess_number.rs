fn guess(_n: i32) -> i32 {
    -1
}

pub unsafe fn guess_number(n: i32) -> i32 {
    let delta = 1.0 + 8.0 * n as f64;
    let solution = (-1.0 + delta.sqrt()) / 2.0;
    let max_interval = solution.ceil() as i64;
    let mut interval = max_interval;
    let mut version: i64 = max_interval;
    let mut before = 1;
    loop {
        let compare = guess(version as i32);
        if compare == 1 {
            interval -= 1;
            before = version;
            version = std::cmp::min(n as i64, version + interval);
        } else {
            for i in before..(version + 1) {
                if guess(i as i32) == 0 {
                    return i as i32;
                }
            }
        }
    }
}

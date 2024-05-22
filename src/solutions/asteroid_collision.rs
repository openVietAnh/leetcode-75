pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![];
    for item in asteroids {
        let mut not_destroyed = true;
        while !stack.is_empty() && item * stack[stack.len() - 1] < 0 && item < 0 {
            let top = stack[stack.len() - 1].abs();
            if item.abs() >= top {
                stack.pop();
                if item.abs() == top {
                    not_destroyed = false;
                    break;
                }
            } else {
                not_destroyed = false;
                break;
            }
        }
        if not_destroyed {
            stack.push(item);
        }
    }
    stack
}

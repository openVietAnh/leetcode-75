pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut stack: Vec<(i32, i32, Vec<i32>, bool)> = vec![(0, 0, vec![], false)];
    while !stack.is_empty() {
        let ind = stack.len() - 1;
        if stack[ind].3 {
            stack.pop();
        } else {
            if stack[ind].0 == k {
                if stack[ind].1 == n {
                    result.push(stack[ind].2.clone());
                }
                stack.pop();
            } else {
                let sum = stack[ind].2.clone();
                let last = if sum.len() == 0 {
                    0
                } else {
                    sum[sum.len() - 1]
                };
                for j in last + 1..=std::cmp::min(n - stack[ind].1, 9) {
                    stack.push((
                        stack[ind].0 + 1,
                        stack[ind].1 + j,
                        [sum.clone(), vec![j]].concat(),
                        false,
                    ))
                }
                stack[ind].3 = true;
            }
        }
    }
    result
}

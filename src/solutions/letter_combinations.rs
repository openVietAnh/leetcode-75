use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let map: HashMap<char, String> = HashMap::from([
        ('2', String::from("abc")),
        ('3', String::from("def")),
        ('4', String::from("ghi")),
        ('5', String::from("jkl")),
        ('6', String::from("mno")),
        ('7', String::from("pqrs")),
        ('8', String::from("tuv")),
        ('9', String::from("wxyz")),
    ]);
    let chars: Vec<char> = digits.chars().collect();
    let mut stack: Vec<(usize, String, bool)> = vec![(0, String::from(""), false)];
    let mut result: Vec<String> = vec![];
    while !stack.is_empty() {
        let last_index = stack.len() - 1;
        if stack[last_index].2 {
            stack.pop();
        } else {
            if stack[last_index].0 == digits.len() {
                if !stack[last_index].1.is_empty() {
                    result.push(stack[last_index].1.clone());
                }
                stack.pop();
            } else {
                for c in map.get(&chars[stack[last_index].0]).unwrap().chars() {
                    stack.push((
                        stack[last_index].0 + 1,
                        stack[last_index].1.clone() + &c.to_string(),
                        false,
                    ));
                }
                stack[last_index].2 = true;
            }
        }
    }
    result
}

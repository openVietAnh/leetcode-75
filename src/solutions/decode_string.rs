pub fn decode_string(s: String) -> String {
    let mut stack: Vec<String> = vec![];
    let mut result = String::from("");
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        if chars[i].is_numeric() {
            let mut number = chars[i].to_string();
            let mut ind = i + 1;
            while ind < chars.len() && chars[ind].is_numeric() {
                number += &chars[ind].to_string();
                ind += 1;
            }
            stack.push(number);
            i = ind;
        } else if chars[i] == ']' {
            let mut cur = String::from("");
            while stack.last().unwrap().parse::<i32>().is_err() {
                cur = stack.pop().unwrap() + &cur;
            }
            let num = stack.pop().unwrap().parse::<usize>().unwrap();
            stack.push(cur.repeat(num));
            i += 1;
        } else if chars[i] != '[' {
            stack.push(chars[i].to_string());
            i += 1;
        } else {
            i += 1;
        }
    }
    stack.into_iter().collect()
}

use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut queue = VecDeque::from_iter(senate.chars());
    loop {
        let (mut ban_radiant, mut ban_dire) = (0, 0);
        let (mut found_radiant, mut found_dire) = (false, false);
        let mut new_queue = VecDeque::new();
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if cur == 'R' {
                if ban_radiant == 0 {
                    ban_dire += 1;
                    found_radiant = true;
                    new_queue.push_back(cur);
                } else {
                    ban_radiant -= 1;
                }
            } else {
                if ban_dire == 0 {
                    ban_radiant += 1;
                    found_dire = true;
                    new_queue.push_back(cur);
                } else {
                    ban_dire -= 1;
                }
            }
        }
        if found_dire && !found_radiant {
            return String::from("Dire");
        }
        if !found_dire && found_radiant {
            return String::from("Radiant");
        }
        queue = new_queue;
    }
}

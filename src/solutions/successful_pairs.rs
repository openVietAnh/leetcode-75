use std::collections::HashMap;

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut pairs: Vec<i32> = vec![];
    let mut count: HashMap<i32, i32> = HashMap::new();
    potions.sort();
    for item in spells {
        if !count.contains_key(&item) {
            let minimum = (success as f64 / item as f64).ceil() as i32;
            match potions.binary_search(&minimum) {
                Ok(ind) => {
                    let mut actual = ind;
                    while actual > 0 && potions[actual - 1] == potions[ind] {
                        actual -= 1;
                    }
                    pairs.push((potions.len() - actual) as i32);
                    count.insert(item, (potions.len() - actual) as i32);
                }
                Err(ind) => {
                    if ind == potions.len() {
                        pairs.push(0);
                        count.insert(item, 0);
                    } else {
                        pairs.push((potions.len() - ind) as i32);
                        count.insert(item, (potions.len() - ind) as i32);
                    }
                }
            }
        } else {
            pairs.push(*count.get(&item).unwrap());
        }
    }
    pairs
}

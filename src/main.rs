use solutions::*;

pub mod solutions;

fn main() {
    println!(
        "{:?}",
        successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
    );
    println!("{:?}", successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16));
}

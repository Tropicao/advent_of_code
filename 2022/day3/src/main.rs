use std::fs;

use rucksacks::compute_priorities_sum;

use crate::rucksacks::compute_badges_sum;

mod rucksacks;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Total priority : {}", compute_priorities_sum(&input));
    println!("Total badges priority : {}", compute_badges_sum(&input));
}

mod strategy;
use std::fs;
use strategy::*;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Total score : {}", compute_score(&input));
    println!("Real total score : {}", compute_real_score(&input));
}

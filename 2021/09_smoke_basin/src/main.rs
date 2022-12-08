mod basin;
use basin::Basin;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = BufReader::new(File::open("inputs.txt").unwrap());
    let input = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let basin = Basin::new(&input);
    println!("Risk levels sum : {}", basin.get_risk_levels_sum());
    println!("Product : {}", basin.get_product_of_three_largest());
}

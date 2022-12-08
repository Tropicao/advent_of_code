mod crabs;
use crabs::Crabs;
use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() {
    let mut reader = BufReader::new(File::open("inputs.txt").unwrap());
    let mut raw_inputs = String::new();
    reader.read_line(&mut raw_inputs).expect("Can not read raw inputs from inputs.txt");
    let crabs = Crabs::new(raw_inputs.split(',').map(|x| x.parse().unwrap()).collect());
    let mut costs = (crabs.min()..crabs.max()).map(|x| crabs.moving_cost_from_target(x)).collect::<Vec<u32>>();
    costs.sort();
    println!("Cost : {}", costs[0]); 
}

mod sea;
use sea::Sea;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(File::open("inputs.txt").unwrap());
    let mut raw_input = String::new();
    reader.read_line(&mut raw_input).expect("Did not manage to read line");
    let sea = Sea::new(raw_input.split(',').map(|x| x.parse().unwrap()).collect());
    println!("There are now {} fishes in the sea", sea.count_fishes_after(256));
}

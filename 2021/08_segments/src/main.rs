mod input;
use input::Input;
use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() {
    let reader = BufReader::new(File::open("inputs.txt").unwrap());
    let result:u32 = reader.lines().map(|x| Input::new(&x.unwrap()).guess_display()).sum();
    println!("Total sum : {}", result);
}

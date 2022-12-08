mod nav;
use nav::Nav;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let nav = Nav::new(
        BufReader::new(File::open("inputs.txt").unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>(),
    );
    println!("Score : {}", nav.score());
    println!("Completion Score : {}", nav.completion_score());
}

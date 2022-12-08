mod sea;
use sea::Sea;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let raw_input = BufReader::new(File::open("inputs.txt").unwrap()).lines();
    let data = raw_input
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut sea = Sea::new(&data);
    println!("Flashes after 100 steps : {}", sea.count_flashes_after(100));
    let mut sea_sync_flash = Sea::new(&data);
    println!("All octopuses flash at step {}", sea_sync_flash.sync_flash_step());
}

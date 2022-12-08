mod calories;
use calories::FoodList;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let elves = FoodList::new(&input).unwrap();
    println!("Elf top calories : {}", elves.get_most_calories());
    println!("Elf top 3 calories : {}", elves.get_top_three_calories());
}

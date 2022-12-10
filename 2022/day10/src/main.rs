mod implementation;
use implementation::*;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 answer : {}", part_1(input));
    println!("Part 2 answer : \n{}", part_2(input));
}

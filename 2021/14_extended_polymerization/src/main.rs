mod polymer;
use polymer::Polymer;
fn main() {
    let mut polymer = Polymer::from_file("inputs.txt");
    for _ in 0..10 {
        polymer.grow();
    }
    println!("Count after 10: {}", polymer.get_diff());
    for _ in 0..30 {
        polymer.grow();
    }
    println!("Count after 40: {}", polymer.get_diff());
}

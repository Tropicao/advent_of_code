mod caves;
use caves::Caves;
fn main() {
    let caves = Caves::from_file("inputs.txt");
    println!("Number of paths : {}", caves.paths_count(false));
    println!("Number of paths (advanced): {}", caves.paths_count(true));
}

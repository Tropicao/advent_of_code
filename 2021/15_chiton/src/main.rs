mod maze;
use maze::Maze;
fn main() {
    let maze = Maze::from_file("inputs.txt");
    println!("Shortest path : {}", maze.get_little_cave_path());
    println!("Shortest path total : {}", maze.get_big_cave_path());
}

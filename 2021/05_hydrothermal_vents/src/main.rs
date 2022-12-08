mod grid;
use grid::Grid;

fn main() {
    let grid = Grid::new("inputs.txt");
    println!("Overlaps : {}", grid.count_overlaps());
}

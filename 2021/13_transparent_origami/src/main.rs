mod origami;
use origami::Origami;
fn main() {
    let mut origami = Origami::from_file("inputs.txt");
    origami.fold_once();
    println!("Points count after 1 fold : {}", origami.count());
    let mut origami = Origami::from_file("inputs.txt");
    origami.fold_completely();
    origami.display();
}

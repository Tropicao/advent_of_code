mod launcher;
use launcher::Launcher;
fn main() {
    let l = Launcher::new(288, 330, -96, -50);
    let (x, y) = l.find_best_shoot_power();
    println!("Best shoot : {}", l.shoot(x, y).unwrap());
    println!("Total shoot count : {}", l.count_shoot_options());
}

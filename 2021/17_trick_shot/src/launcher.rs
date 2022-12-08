mod coord;
use coord::{Coord, Zone};
use std::collections::HashMap;
pub struct Launcher {
    target: Zone
}

impl Launcher {
    pub fn new(x_min: i32, x_max: i32, y_min: i32, y_max:i32) -> Self
    {
        Launcher {
            target: Zone::new(x_min, x_max, y_min, y_max)
        }
    }

    pub fn shoot(&self, x_speed: i32, y_speed:i32) -> Option<i32> {
        let mut p = Coord::new(0, 0, x_speed, y_speed);
        while p.y >= self.target.y_min && !self.target.contains(&p) {
            p.move_probe();
        }
        if self.target.contains(&p) {
            return Some(p.max_y);
        }
        None
    }

    fn list_all_shoot_options(&self) -> HashMap<(i32, i32), i32> {
        let x_max = self.target.x_max + 1;
        let y_max = self.target.y_min.abs();
        let mut candidates = HashMap::new();
        for x in 0..x_max {
            for y in -y_max..y_max {
                if let Some(h) = self.shoot(x, y) {
                    candidates.insert((x, y), h);
                }
            }
        }
        candidates
    }

    pub fn find_best_shoot_power(&self) -> (i32, i32)
    {
        *self.list_all_shoot_options().iter().max_by(|((_, _), h1), ((_, _), h2)| h1.cmp(h2)).map(|(k, _)| k).unwrap()
    }

    pub fn count_shoot_options(&self) -> usize {
        self.list_all_shoot_options().len()
    }
}

#[cfg(test)]
mod tests {
    use super::Launcher;

    #[test]
    fn test_shoot_in() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.shoot(7, 2), Some(3));
    }

    #[test]
    fn test_shoot_in_bis() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.shoot(6, 3), Some(6));
    }

    #[test]
    fn test_shoot_in_bis_bis() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.shoot(9, 0), Some(0));
    }

    #[test]
    fn test_shoot_out() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.shoot(17, -4), None);
    }

    #[test]
    fn test_shoot_best() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.shoot(6, 9), Some(45));
    }

    #[test]
    fn test_find_best_shoot_power() {
        let l = Launcher::new(20, 30, -10, -5);
        let (x, y) = l.find_best_shoot_power();
        assert!(y == 9 && (x==6 || x == 7));
    }

    #[test]
    fn test_count_options() {
        let l = Launcher::new(20, 30, -10, -5);
        assert_eq!(l.count_shoot_options(),112);
    }
    #[test]
    fn test_some_shoots()
    {
        let l = Launcher::new(20, 30, -10, -5);
        assert_ne!(l.shoot(23, -10), None);
        assert_ne!(l.shoot(23,-10), None);
        assert_ne!(l.shoot(25,-9), None);
        assert_ne!(l.shoot(27,-5), None);
        assert_ne!(l.shoot(29,-6), None);
        assert_ne!(l.shoot(22,-6), None);
        assert_ne!(l.shoot(21,-7), None);
        assert_ne!(l.shoot(9,0), None);
        assert_ne!(l.shoot(27,-7), None);
        assert_ne!(l.shoot(24,-5), None);
        assert_ne!(l.shoot(25,-7), None);
        assert_ne!(l.shoot(26,-6), None);
        assert_ne!(l.shoot(25,-5), None);
        assert_ne!(l.shoot(6,8), None);
        assert_ne!(l.shoot(11,-2), None);
        assert_ne!(l.shoot(20,-5), None);
        assert_ne!(l.shoot(29,-10), None);
        assert_ne!(l.shoot(6,3), None);
        assert_ne!(l.shoot(28,-7), None);
        assert_ne!(l.shoot(8,0), None);
        assert_ne!(l.shoot(30,-6), None);
        assert_ne!(l.shoot(29,-8), None);
        assert_ne!(l.shoot(20,-10), None);
        assert_ne!(l.shoot(6,7), None);
        assert_ne!(l.shoot(6,4), None);
        assert_ne!(l.shoot(6,1), None);
        assert_ne!(l.shoot(14,-4), None);
        assert_ne!(l.shoot(21,-6), None);
        assert_ne!(l.shoot(26,-10), None);
        assert_ne!(l.shoot(7,-1), None);
        assert_ne!(l.shoot(7,7), None);
        assert_ne!(l.shoot(8,-1), None);
        assert_ne!(l.shoot(21,-9), None);
        assert_ne!(l.shoot(6,2), None);
        assert_ne!(l.shoot(20,-7), None);
        assert_ne!(l.shoot(30,-10), None);
        assert_ne!(l.shoot(14,-3), None);
        assert_ne!(l.shoot(20,-8), None);
        assert_ne!(l.shoot(13,-2), None);
        assert_ne!(l.shoot(7,3), None);
        assert_ne!(l.shoot(28,-8), None);
        assert_ne!(l.shoot(29,-9), None);
        assert_ne!(l.shoot(15,-3), None);
        assert_ne!(l.shoot(22,-5), None);
        assert_ne!(l.shoot(26,-8), None);
        assert_ne!(l.shoot(25,-8), None);
        assert_ne!(l.shoot(25,-6), None);
        assert_ne!(l.shoot(15,-4), None);
        assert_ne!(l.shoot(9,-2), None);
        assert_ne!(l.shoot(15,-2), None);
        assert_ne!(l.shoot(12,-2), None);
        assert_ne!(l.shoot(28,-9), None);
        assert_ne!(l.shoot(12,-3), None);
        assert_ne!(l.shoot(24,-6), None);
        assert_ne!(l.shoot(23,-7), None);
        assert_ne!(l.shoot(25,-10), None);
        assert_ne!(l.shoot(7,8), None);
        assert_ne!(l.shoot(11,-3), None);
        assert_ne!(l.shoot(26,-7), None);
        assert_ne!(l.shoot(7,1), None);
        assert_ne!(l.shoot(23,-9), None);
        assert_ne!(l.shoot(6,0), None);
        assert_ne!(l.shoot(22,-10), None);
        assert_ne!(l.shoot(27,-6), None);
        assert_ne!(l.shoot(8,1), None);
        assert_ne!(l.shoot(22,-8), None);
        assert_ne!(l.shoot(13,-4), None);
        assert_ne!(l.shoot(7,6), None);
        assert_ne!(l.shoot(28,-6), None);
        assert_ne!(l.shoot(11,-4), None);
        assert_ne!(l.shoot(12,-4), None);
        assert_ne!(l.shoot(26,-9), None);
        assert_ne!(l.shoot(7,4), None);
        assert_ne!(l.shoot(24,-10), None);
        assert_ne!(l.shoot(23,-8), None);
        assert_ne!(l.shoot(30,-8), None);
        assert_ne!(l.shoot(7,0), None);
        assert_ne!(l.shoot(9,-1), None);
        assert_ne!(l.shoot(10,-1), None);
        assert_ne!(l.shoot(26,-5), None);
        assert_ne!(l.shoot(22,-9), None);
        assert_ne!(l.shoot(6,5), None);
        assert_ne!(l.shoot(7,5), None);
        assert_ne!(l.shoot(23,-6), None);
        assert_ne!(l.shoot(28,-10), None);
        assert_ne!(l.shoot(10,-2), None);
        assert_ne!(l.shoot(11,-1), None);
        assert_ne!(l.shoot(20,-9), None);
        assert_ne!(l.shoot(14,-2), None);
        assert_ne!(l.shoot(29,-7), None);
        assert_ne!(l.shoot(13,-3), None);
        assert_ne!(l.shoot(23,-5), None);
        assert_ne!(l.shoot(24,-8), None);
        assert_ne!(l.shoot(27,-9), None);
        assert_ne!(l.shoot(30,-7), None);
        assert_ne!(l.shoot(28,-5), None);
        assert_ne!(l.shoot(21,-10), None);
        assert_ne!(l.shoot(7,9), None);
        assert_ne!(l.shoot(6,6), None);
        assert_ne!(l.shoot(21,-5), None);
        assert_ne!(l.shoot(27,-10), None);
        assert_ne!(l.shoot(7,2), None);
        assert_ne!(l.shoot(30,-9), None);
        assert_ne!(l.shoot(21,-8), None);
        assert_ne!(l.shoot(22,-7), None);
        assert_ne!(l.shoot(24,-9), None);
        assert_ne!(l.shoot(20,-6), None);
        assert_ne!(l.shoot(6,9), None);
        assert_ne!(l.shoot(29,-5), None);
        assert_ne!(l.shoot(8,-2), None);
        assert_ne!(l.shoot(27,-8), None);
        assert_ne!(l.shoot(30,-5), None);
        assert_ne!(l.shoot(24,-7), None);
    }

    #[test]
    fn compare_close_shoots() {
        let l = Launcher::new(20, 30, -10, -5);
        println!("6, 9 : {}", l.shoot(6, 9).unwrap());
        println!("7, 9 : {}", l.shoot(7, 9).unwrap());
    }
}
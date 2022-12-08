pub struct Coord {
    x: i32,
    pub y: i32,
    x_speed: i32,
    y_speed: i32,
    pub max_y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32, x_speed: i32, y_speed: i32) -> Self {
        Coord { x, y, x_speed, y_speed, max_y: y }
    }

    pub fn move_probe(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;
        self.x_speed = match self.x_speed.cmp(&0) {
            std::cmp::Ordering::Greater => self.x_speed - 1,
            std::cmp::Ordering::Less => self.x_speed + 1,
            std::cmp::Ordering::Equal => 0 
        };
        self.y_speed -=1;
        if self.y > self.max_y {
            self.max_y = self.y;
        }
    }
}

pub struct Zone {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl Zone {
    pub fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Zone {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn contains(&self, c: &Coord) -> bool {
        c.x >= self.x_min && c.x <= self.x_max && c.y >= self.y_min && c.y <= self.y_max
    }
}

#[cfg(test)]
mod tests {
    use super::{Coord, Zone};
    #[test]
    fn test_coord_top_of_zone() {
        let c = Coord::new(25, 5, 0, 0);
        let z = Zone::new(20, 30, -10, -5);
        assert!(!z.contains(&c));
    }
    #[test]
    fn test_coord_lower_than_zone() {
        let c = Coord::new(25, -15, 0, 0);
        let z = Zone::new(20, 30, -10, -5);
        assert!(!z.contains(&c));
    }
    #[test]
    fn test_coord_left_of_zone() {
        let c = Coord::new(15, -8, 0, 0);
        let z = Zone::new(20, 30, -10, -5);
        assert!(!z.contains(&c));
    }
    #[test]
    fn test_coord_right_of_zone() {
        let c = Coord::new(35, -8, 0, 0);
        let z = Zone::new(20, 30, -10, -5);
        assert!(!z.contains(&c));
    }
    #[test]
    fn test_coord_in_zone() {
        let c = Coord::new(25, -8, 0, 0);
        let z = Zone::new(20, 30, -10, -5);
        assert!(z.contains(&c));
    }

    #[test]
    fn test_move() {
        let mut c1 = Coord::new(0, 0, 5, 3);
        c1.move_probe();
        assert_eq!(c1.x, 5);
        assert_eq!(c1.y, 3);
        assert_eq!(c1.x_speed, 4);
        assert_eq!(c1.y_speed, 2);
    }
    #[test]
    fn test_move_rev_x() {
        let mut c1 = Coord::new(0, 0, -5, 3);
        c1.move_probe();
        assert_eq!(c1.x, -5);
        assert_eq!(c1.y, 3);
        assert_eq!(c1.x_speed, -4);
        assert_eq!(c1.y_speed, 2);
    }
    #[test]
    fn test_move_no_rx() {
        let mut c1 = Coord::new(0, 0, 0, 3);
        c1.move_probe();
        assert_eq!(c1.x, 0);
        assert_eq!(c1.y, 3);
        assert_eq!(c1.x_speed, 0);
        assert_eq!(c1.y_speed, 2);
    }
    #[test]
    fn test_record_max_height() {
        let mut c1 = Coord::new(0, 0, 7, 2);
        c1.move_probe();
        c1.move_probe();
        c1.move_probe();
        c1.move_probe();
        c1.move_probe();
        c1.move_probe();
        assert_eq!(c1.max_y, 3);
    }
}

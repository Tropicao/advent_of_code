#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}
#[derive(Copy, Clone)]
pub enum FoldAxis {
    X,
    Y,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn fold(self, axis: usize, axis_type: FoldAxis) -> Self {
        match axis_type {
            FoldAxis::X => Point {
                x: if axis < self.x {
                    2 * axis - self.x
                } else {
                    self.x
                },
                y: self.y,
            },
            FoldAxis::Y => Point {
                x: self.x,
                y: if axis < self.y {
                    2*axis - self.y
                } else {
                    self.y
                },
            },
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::{FoldAxis, Point};
    #[test]
    pub fn test_fold_x_unchanged() {
        assert_eq!(Point::new(2, 2).fold(4, FoldAxis::X), Point::new(2, 2));
    }
    #[test]
    pub fn test_fold_y_unchanged() {
        assert_eq!(Point::new(2, 2).fold(4, FoldAxis::Y), Point::new(2, 2));
    }
    #[test]
    pub fn test_fold_x_changed() {
        assert_eq!(Point::new(2, 2).fold(1, FoldAxis::X), Point::new(0, 2));
    }
    #[test]
    pub fn test_fold_y_changed() {
        assert_eq!(Point::new(2, 2).fold(1, FoldAxis::Y), Point::new(2, 0));
    }
    #[test]
    pub fn test_fold_arbitrary() {
        assert_eq!(Point::new(0, 14).fold(7, FoldAxis::Y), Point::new(0, 0));
    }
    #[test]
    pub fn test_fold_arbitrary_2() {
        assert_eq!(Point::new(1, 10).fold(7, FoldAxis::Y), Point::new(1, 4));
    }
}

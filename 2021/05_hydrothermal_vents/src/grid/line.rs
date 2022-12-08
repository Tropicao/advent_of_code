use std::cmp::Ordering;
pub struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    pub fn new(start: (u32, u32), end: (u32, u32)) -> Self {
        println!("new line : {:?}:{:?}", start, end);
        Line { start, end }
    }

    pub fn get_occupied_points(&self) -> Vec<(u32, u32)> {
        if self.start.0 == self.end.0 {
            let (min, max) = match self.start.1.cmp(&self.end.1) {
                Ordering::Less => (self.start.1, self.end.1),
                _ => (self.end.1, self.start.1),
            };
            (min..=max)
                .map(|x| (self.start.0, x))
                .collect::<Vec<(u32, u32)>>()
        } else if self.start.1 == self.end.1 {
            let (min, max) = match self.start.0.cmp(&self.end.0) {
                Ordering::Less => (self.start.0, self.end.0),
                _ => (self.end.0, self.start.0),
            };
            (min..=max)
                .map(|x| (x, self.start.1))
                .collect::<Vec<(u32, u32)>>()
        } else {
            let (new_start, new_end) = match self.start.0.cmp(&self.end.0) {
                Ordering::Less => (self.start, self.end),
                _ => (self.end, self.start),
            };
            let y_increase = new_start.1 < new_end.1;
            let mut current_y = new_start.1;
            (new_start.0..=new_end.0)
                .map(|x| {
                    (
                        x,
                        match y_increase {
                            true => {
                                let result = current_y;
                                current_y += 1;
                                result
                            }
                            false => {
                                let result = current_y;
                                current_y -= 1;
                                result
                            }
                        },
                    )
                })
                .collect::<Vec<(u32, u32)>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    #[test]
    fn test_get_occupied_points_horizontal() {
        let line = Line::new((1, 1), (3, 1));
        assert_eq!(line.get_occupied_points(), vec![(1, 1), (2, 1), (3, 1)])
    }

    #[test]
    fn test_get_occupied_points_horizontal_reverse() {
        let line = Line::new((3, 1), (1, 1));
        assert_eq!(line.get_occupied_points(), vec![(1, 1), (2, 1), (3, 1)])
    }

    #[test]
    fn test_get_occupied_points_vertical() {
        let line = Line::new((1, 1), (1, 3));
        assert_eq!(line.get_occupied_points(), vec![(1, 1), (1, 2), (1, 3)])
    }

    #[test]
    fn test_get_occupied_points_vertical_reverse() {
        let line = Line::new((1, 3), (1, 1));
        assert_eq!(line.get_occupied_points(), vec![(1, 1), (1, 2), (1, 3)])
    }

    #[test]
    fn test_get_occupied_points_diagonal_left_right() {
        let mut result = Line::new((1, 1), (3, 3)).get_occupied_points();
        result.sort();
        let mut expected = vec![(1, 1), (2, 2), (3, 3)];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_occupied_points_diagonal_left_right_reverse() {
        let mut result = Line::new((3, 3), (1, 1)).get_occupied_points();
        result.sort();
        let mut expected = vec![(1, 1), (2, 2), (3, 3)];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_occupied_points_diagonal_right_left() {
        let mut result = Line::new((3, 1), (1, 3)).get_occupied_points();
        result.sort();
        let mut expected = vec![(1, 3), (2, 2), (3, 1)];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_occupied_points_diagonal_right_left_reverse() {
        let mut result = Line::new((1, 3), (3, 1)).get_occupied_points();
        result.sort();
        let mut expected = vec![(1, 3), (2, 2), (3, 1)];
        expected.sort();
        assert_eq!(result, expected);
    }
}

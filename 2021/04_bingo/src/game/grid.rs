mod cell;
use cell::Cell;

#[derive(PartialEq, Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(values: Vec<Vec<u32>>) -> Self {
        let mut cells = Vec::new();
        for v in values {
            let mut line = Vec::new();
            for w in v {
                line.push(Cell::new(w));
            }
            cells.push(line);
        }
        Grid { cells }
    }

    pub fn draw(&mut self, value: u32) {
        for i in self.cells.iter_mut() {
            for v in i {
                if v.value() == value {
                    v.check();
                }
            }
        }
    }

    pub fn is_winning(&self) -> bool {
        // Two conditions :
        // * any line is completely checked
        // * build a vec of columns value and check if any column is completely checked
        self.cells.iter().any(|x| x.iter().all(|y| y.is_checked()))
            || (0..self.cells[0].len())
                .map(|x| self.cells.iter().map(|y| y[x]).collect::<Vec<Cell>>())
                .any(|z| z.iter().all(|a| a.is_checked()))
    }

    pub fn get_unmarked_sum(&self) -> u32 {
        self.cells
            .iter()
            .flatten()
            .filter_map(|c| {
                if !c.is_checked() {
                    Some(c.value())
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use super::Grid;
    #[test]
    fn test_new_grid() {
        let input = vec![vec![0, 1], vec![2, 3]];
        let expected = Grid {
            cells: vec![
                vec![Cell::new(0), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        assert_eq!(Grid::new(input), expected);
    }

    #[test]
    fn test_draw_no_winning() {
        let mut grid = Grid {
            cells: vec![
                vec![Cell::new(0), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        grid.draw(0);
        assert!(!grid.is_winning());
    }

    #[test]
    fn test_draw_cell_checked() {
        let mut grid = Grid {
            cells: vec![
                vec![Cell::new(0), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        grid.draw(2);
        assert!(grid.cells[1][0].is_checked())
    }

    #[test]
    fn test_winning_line() {
        let mut grid = Grid {
            cells: vec![
                vec![Cell::new(0), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        grid.draw(2);
        grid.draw(3);
        assert!(grid.is_winning());
    }

    #[test]
    fn test_winning_column() {
        let mut grid = Grid {
            cells: vec![
                vec![Cell::new(0), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        grid.draw(2);
        grid.draw(0);
        assert!(grid.is_winning());
    }

    #[test]
    fn test_get_unmarked() {
        let mut grid = Grid {
            cells: vec![
                vec![Cell::new(5), Cell::new(1)],
                vec![Cell::new(2), Cell::new(3)],
            ],
        };
        grid.draw(1);
        grid.draw(3);
        assert_eq!(grid.get_unmarked_sum(), 7);
    }
}

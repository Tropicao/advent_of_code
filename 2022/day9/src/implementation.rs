use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord(isize, isize);

impl Coord {
    fn move_coord(&mut self, movement: Move) {
        match movement {
            Move::Up => self.1 += 1,
            Move::Down => self.1 -= 1,
            Move::Left => self.0 -= 1,
            Move::Right => self.0 += 1,
            Move::UpRight => {
                self.0 += 1;
                self.1 += 1
            }
            Move::UpLeft => {
                self.0 -= 1;
                self.1 += 1
            }
            Move::DownRight => {
                self.0 += 1;
                self.1 -= 1
            }
            Move::DownLeft => {
                self.0 -= 1;
                self.1 -= 1
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Debug)]
struct Instruction {
    movement: Move,
    count: usize,
}
struct Grid {
    knots: Vec<Coord>,
    visited: Vec<HashSet<Coord>>,
    moves: Vec<Instruction>,
}

impl Grid {
    const KNOTS_COUNT: usize = 10;
    fn new(input: &str) -> Self {
        Grid {
            knots: vec![Coord(0, 0); Grid::KNOTS_COUNT],
            visited: vec![HashSet::from([Coord(0, 0)]); Grid::KNOTS_COUNT],
            moves: input
                .lines()
                .map(|x| x.split_ascii_whitespace().collect::<Vec<&str>>())
                .map(|x| match x[0] {
                    "U" => Instruction {
                        movement: Move::Up,
                        count: x[1].parse::<usize>().unwrap(),
                    },
                    "D" => Instruction {
                        movement: Move::Down,
                        count: x[1].parse::<usize>().unwrap(),
                    },
                    "L" => Instruction {
                        movement: Move::Left,
                        count: x[1].parse::<usize>().unwrap(),
                    },
                    "R" => Instruction {
                        movement: Move::Right,
                        count: x[1].parse::<usize>().unwrap(),
                    },
                    _ => panic!("Unexpected move char {}", x[0]),
                })
                .collect::<Vec<Instruction>>(),
        }
    }

    fn get_knot_move(&self, i: usize) -> Option<Move> {
        // Tail is still close enough to head : no need to move
        assert!(i > 0);
        if self.knots[i - 1].0.abs_diff(self.knots[i].0) <= 1
            && self.knots[i - 1].1.abs_diff(self.knots[i].1) <= 1
        {
            return None;
        }
        if self.knots[i - 1].0 > self.knots[i].0 && self.knots[i - 1].1 > self.knots[i].1 {
            return Some(Move::UpRight);
        }
        if self.knots[i - 1].0 < self.knots[i].0 && self.knots[i - 1].1 > self.knots[i].1 {
            return Some(Move::UpLeft);
        }
        if self.knots[i - 1].0 > self.knots[i].0 && self.knots[i - 1].1 < self.knots[i].1 {
            return Some(Move::DownRight);
        }
        if self.knots[i - 1].0 < self.knots[i].0 && self.knots[i - 1].1 < self.knots[i].1 {
            return Some(Move::DownLeft);
        }
        if self.knots[i - 1].0 > self.knots[i].0 {
            return Some(Move::Right);
        }
        if self.knots[i - 1].0 < self.knots[i].0 {
            return Some(Move::Left);
        }
        if self.knots[i - 1].1 > self.knots[i].1 {
            return Some(Move::Up);
        }
        if self.knots[i - 1].1 < self.knots[i].1 {
            return Some(Move::Down);
        }
        None
    }
    fn run_moves(&mut self) {
        for m in &self.moves[..] {
            for _ in 0..m.count {
                self.knots[0].move_coord(m.movement);
                for i in 1..Grid::KNOTS_COUNT {
                    if let Some(m) = self.get_knot_move(i) {
                        self.knots[i].move_coord(m);
                        self.visited[i].insert(self.knots[i]);
                    }
                }
            }
        }
    }

    fn count_visited(&self, i: usize) -> usize {
        self.visited[i].len()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.run_moves();
    grid.count_visited(1)
}

pub fn part_2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.run_moves();
    grid.count_visited(9)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    // Set test input in this variable
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_LARGER_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test_case(1, 1, Move::Up, 1, 2; "up")]
    #[test_case(1, 1, Move::Down, 1, 0; "down")]
    #[test_case(1, 1, Move::Left, 0, 1; "left")]
    #[test_case(1, 1, Move::Right, 2, 1; "right")]
    #[test_case(1, 1, Move::UpLeft, 0, 2; "up-left")]
    #[test_case(1, 1, Move::UpRight, 2, 2; "up-right")]
    #[test_case(1, 1, Move::DownLeft, 0, 0; "down-left")]
    #[test_case(1, 1, Move::DownRight, 2, 0; "down-right")]
    fn test_move_coord(old_x: isize, old_y: isize, movement: Move, new_x: isize, new_y: isize) {
        let mut coord = Coord(old_x, old_y);
        coord.move_coord(movement);
        assert_eq!(coord.0, new_x);
        assert_eq!(coord.1, new_y);
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 13)
    }

    #[test_case(TEST_INPUT, 1; "Small grid")]
    #[test_case(TEST_LARGER_INPUT, 36; "Large grid")]
    fn test_part_2(input: &str, output: usize) {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(input), output)
    }
}

use std::{cmp, collections::HashSet, fmt::Display, hash::Hash};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    sequence::separated_pair,
};

#[derive(PartialEq)]
enum CellKind {
    Empty,
    Rock,
    Sand,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x, y }
    }
}

fn rocks(a: &Coord, b: &Coord) -> Vec<Coord> {
    if a.x == b.x {
        (cmp::min(a.y, b.y)..=cmp::max(a.y, b.y))
            .map(|y| Coord::from((a.x, y)))
            .collect()
    } else if a.y == b.y {
        (cmp::min(a.x, b.x)..=cmp::max(a.x, b.x))
            .map(|x| Coord::from((x, a.y)))
            .collect()
    } else {
        panic!("I can not draw lines that are not horizontal nor vertical");
    }
}

#[derive(Default)]
struct Cave {
    rocks: HashSet<Coord>,
    sand: HashSet<Coord>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
    floored: bool,
}

fn parse_rocks(input: &str) -> HashSet<Coord> {
    let coords = separated_list1(
        tag(" -> "),
        map(
            separated_pair(digit1::<&str, nom::error::Error<&str>>, tag(","), digit1),
            |(x, y)| Coord::from((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())),
        ),
    )(input)
    .unwrap()
    .1;
    coords
        .windows(2)
        .flat_map(|c| rocks(&c[0], &c[1]))
        .collect::<HashSet<Coord>>()
}

impl Cave {
    const SAND_SOURCE_X: usize = 500;
    const SAND_SOURCE_Y: usize = 0;
    fn new(input: &str) -> Self {
        let mut cave = Cave {
            rocks: input.lines().fold(HashSet::new(), |acc, l| {
                acc.union(&parse_rocks(l)).copied().collect::<HashSet<_>>()
            }),
            ..Default::default()
        };
        cave.x_min = cave
            .rocks
            .iter()
            .reduce(|acc, item| if item.x < acc.x { item } else { acc })
            .unwrap()
            .x;
        cave.x_max = cave
            .rocks
            .iter()
            .reduce(|acc, item| if item.x > acc.x { item } else { acc })
            .unwrap()
            .x;
        cave.y_max = cave
            .rocks
            .iter()
            .reduce(|acc, item| if item.y > acc.y { item } else { acc })
            .unwrap()
            .y;
        cave
    }

    fn is_in_bound(&self, c: &Coord) -> bool {
        c.x >= self.x_min && c.x <= self.x_max && c.y <= self.y_max
    }

    fn get_cell_type(&self, c: Coord) -> CellKind {
        if self.rocks.contains(&c) || (self.floored && c.y == self.y_max + 2) {
            CellKind::Rock
        } else if self.sand.contains(&c) {
            CellKind::Sand
        } else {
            CellKind::Empty
        }
    }

    fn get_down_cell(&self, c: &Coord) -> Coord {
        Coord::from((c.x, c.y + 1))
    }

    fn get_down_left_cell(&self, c: &Coord) -> Coord {
        Coord::from((c.x - 1, c.y + 1))
    }

    fn get_down_right_cell(&self, c: &Coord) -> Coord {
        Coord::from((c.x + 1, c.y + 1))
    }

    fn move_sand(&self, c: Coord) -> Coord {
        let down = self.get_down_cell(&c);
        if self.get_cell_type(down) == CellKind::Empty {
            return down;
        }
        let down_left = self.get_down_left_cell(&c);
        if self.get_cell_type(down_left) == CellKind::Empty {
            return down_left;
        }
        let down_right = self.get_down_right_cell(&c);
        if self.get_cell_type(down_right) == CellKind::Empty {
            return down_right;
        }
        c
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = Coord::from((Self::SAND_SOURCE_X, Self::SAND_SOURCE_Y));
        loop {
            let new_sand = self.move_sand(sand);
            if new_sand == sand {
                self.sand.insert(sand);
                return false;
            } else if !self.floored && !self.is_in_bound(&new_sand) {
                return true;
            } else {
                sand = new_sand;
            }
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for j in 0..=self.y_max {
            if j > 0 {
                result.push('\n');
            }
            for i in self.x_min..=self.x_max {
                if self.rocks.contains(&Coord::from((i, j))) {
                    result.push('#');
                } else if self.sand.contains(&Coord::from((i, j))) {
                    result.push('o');
                } else if Coord::from((i, j))
                    == Coord::from((Cave::SAND_SOURCE_X, Cave::SAND_SOURCE_Y))
                {
                    result.push('+');
                } else {
                    result.push('.');
                }
            }
        }
        write!(f, "{result}")
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cave = Cave::new(input);
    while !cave.drop_sand() {}
    println!("{cave}");
    cave.sand.len()
}

pub fn part_2(input: &str) -> usize {
    let mut cave = Cave::new(input);
    cave.floored = true;
    while !cave
        .sand
        .contains(&Coord::from((Cave::SAND_SOURCE_X, Cave::SAND_SOURCE_Y)))
    {
        cave.drop_sand();
    }
    println!("{cave}");
    cave.sand.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_data;
    use test_data::*;

    #[test]
    fn test_horizontal_line() {
        let (a, b) = (Coord::from((1, 1)), Coord::from((3, 1)));
        assert_eq!(
            rocks(&a, &b),
            vec![
                Coord::from((1, 1)),
                Coord::from((2, 1)),
                Coord::from((3, 1))
            ]
        );
    }

    #[test]
    fn test_vertical_line() {
        let (a, b) = (Coord::from((1, 1)), Coord::from((1, 3)));
        assert_eq!(
            rocks(&a, &b),
            vec![
                Coord::from((1, 1)),
                Coord::from((1, 2)),
                Coord::from((1, 3))
            ]
        );
    }

    #[test]
    fn test_load_cave() {
        let cave = Cave::new(TEST_INPUT);
        assert_eq!(format!("{cave}"), TEST_OUTPUT_CAVE);
    }

    #[test]
    fn test_drop_one() {
        let mut cave = Cave::new(TEST_INPUT);
        cave.drop_sand();
        assert_eq!(format!("{cave}"), TEST_OUTPUT_DROP_ONE_SAND);
    }

    #[test]
    fn test_drop_two() {
        let mut cave = Cave::new(TEST_INPUT);
        cave.drop_sand();
        cave.drop_sand();
        assert_eq!(format!("{cave}"), TEST_OUTPUT_DROP_TWO_SAND);
    }

    #[test]
    fn test_drop_five() {
        let mut cave = Cave::new(TEST_INPUT);
        for _ in 0..5 {
            cave.drop_sand();
        }
        assert_eq!(format!("{cave}"), TEST_OUTPUT_DROP_FIVE_SAND);
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 24)
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 93)
    }
}

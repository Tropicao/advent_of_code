use std::{collections::HashSet, hash::Hash};

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

#[derive(Default)]
struct Grid {
    squares: Vec<Vec<char>>,
    src: Coord,
    dst: Coord,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut result = Grid {
            squares: input
                .lines()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect(),
            ..Default::default()
        };
        for (j, line) in result.squares.iter_mut().enumerate() {
            for (i, c) in line.iter_mut().enumerate() {
                if *c == 'S' {
                    result.src = Coord::new(i, j);
                    *c = 'a';
                } else if *c == 'E' {
                    result.dst = Coord::new(i, j);
                    *c = 'z';
                }
            }
        }
        result
    }

    fn reachable(&self, c: Coord, n: Coord) -> bool {
        self.squares[c.y][c.x] as usize + 1 >= self.squares[n.y][n.x] as usize
    }

    fn get_neigh_coord_raw(&self, c: Coord) -> Vec<Coord> {
        let mut result = vec![];
        if c.x > 0 {
            let n = Coord::new(c.x - 1, c.y);
            if self.reachable(c, n) {
                result.push(n);
            }
        }
        if c.x < self.squares[0].len() - 1 {
            let n = Coord::new(c.x + 1, c.y);
            if self.reachable(c, n) {
                result.push(n);
            }
        }
        if c.y > 0 {
            let n = Coord::new(c.x, c.y - 1);
            if self.reachable(c, n) {
                {
                    result.push(n);
                }
            }
        }
        if c.y < self.squares.len() - 1 {
            let n = Coord::new(c.x, c.y + 1);
            if self.reachable(c, n) {
                {
                    result.push(n);
                }
            }
        }
        result
    }

    fn get_dist_to_dst(&self, c: Coord) -> u64 {
        ((c.x.abs_diff(self.dst.x).pow(2) + c.y.abs_diff(self.dst.y).pow(2)) as f64).sqrt() as u64
    }

    fn get_cheapest_path(&self, start: Coord, threshold: Option<usize>) -> usize {
        let mut open = HashSet::new();
        let mut close = open.clone();
        open.insert((start, 0, 0));
        while !open.is_empty() {
            let candidate = open
                .iter()
                .cloned()
                .reduce(|acc, item| if acc.2 < item.2 { acc } else { item })
                .unwrap();
            open.remove(&candidate);
            if candidate.0 == self.dst {
                return candidate.1;
            };
            if let Some(x) = threshold {
                if candidate.1 >= x {
                    return usize::MAX;
                }
            }
            for n in self.get_neigh_coord_raw(candidate.0) {
                if close.iter().cloned().any(|x| x.0 == n) {
                    continue;
                }
                let cost = candidate.1 + 1;
                if open.iter().cloned().any(|x| x.0 == n && x.1 < cost) {
                    continue;
                }
                let h = cost as u64 + self.get_dist_to_dst(n);
                open.insert((n, cost, h));
            }
            close.insert(candidate);
        }
        0
    }
}

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.get_cheapest_path(grid.src, None)
}

pub fn part_2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut candidates = vec![(grid.src, grid.get_dist_to_dst(grid.src))];
    for (j, line) in grid.squares.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if *c == 'a' {
                let candidate = Coord::new(i, j);
                candidates.push((candidate, grid.get_dist_to_dst(candidate)));
            }
        }
    }
    
    // Check all candidates : pick best choice, compute path, stop if costs under computation is higher than minimal path found
    let mut shortest = grid.get_cheapest_path(grid.src, Some(usize::MAX));
    while !candidates.is_empty() {
        let (index, _) = candidates
            .iter()
            .enumerate()
            .reduce(|acc, item| if acc.1.1 < item.1.1 { acc } else { item })
            .unwrap();
        let candidate = candidates.swap_remove(index).0;
        let path_cost = grid.get_cheapest_path(candidate, Some(shortest));
        println!("Cost for {candidate:?} : {path_cost}.\t {} remanining, shortest is {shortest}", candidates.len());
        if path_cost > 0 && path_cost < shortest {
            shortest = path_cost;
        }
    }
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;
    // Set test input in this variable
    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_grid_parsing() {
        let grid = Grid::new(TEST_INPUT);
        assert_eq!(grid.src, Coord::new(0, 0));
        assert_eq!(grid.dst, Coord::new(5, 2));
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 31)
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 29)
    }
}

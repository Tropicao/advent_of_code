mod line;
use line::Line;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Grid {
    lines: Vec<Line>,
}

impl Grid {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines: Vec<Line> = Vec::new();
        for l in reader.lines() {
            let parts = l.unwrap();
            let split_parts: Vec<&str> = parts.trim().split("->").collect();
            let start: Vec<&str> = split_parts[0].trim().split(',').collect();
            let end: Vec<&str> = split_parts[1].trim().split(',').collect();
            lines.push(Line::new(
                (start[0].parse().unwrap(), start[1].parse().unwrap()),
                (end[0].parse().unwrap(), end[1].parse().unwrap()),
            ));
        }
        Grid { lines }
    }

    pub fn count_overlaps(&self) -> usize {
        let mut hash = HashMap::new();
        let occupied_points: Vec<(u32, u32)> = self
            .lines
            .iter()
            .map(|x| x.get_occupied_points())
            .flatten()
            .collect();
        for point in occupied_points {
            let entry = hash.entry(point).or_insert(0);
            *entry += 1;
        }
        hash.into_iter().filter(|(_, val)| val > &1).count()
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    #[test]
    fn test_load_lines() {
        let grid = Grid::new("inputs_test.txt");
        assert_eq!(grid.lines.len(), 6);
    }

    #[test]
    fn test_count_overlaps() {
        let grid = Grid::new("inputs_test.txt");
        assert_eq!(grid.count_overlaps(), 5);
    }
}

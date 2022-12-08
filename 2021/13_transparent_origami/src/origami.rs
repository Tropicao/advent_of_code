mod point;
use point::{FoldAxis, Point};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Origami {
    points: HashSet<Point>,
    folds: Vec<(usize, FoldAxis)>,
}

impl Origami {
    pub fn from_file(path: &str) -> Self {
        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut points = HashSet::new();
        let mut folds = Vec::new();
        let mut line = String::new();
        loop {
            reader
                .read_line(&mut line)
                .expect("Did not manage to read line");
            if !line.trim().is_empty() {
                let raw_point = line
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                points.insert(Point::new(raw_point[0], raw_point[1]));
                line.clear();
            } else {
                break;
            }
        }
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let raw_fold = line
                        .trim()
                        .split(' ')
                        .last()
                        .unwrap()
                        .split('=')
                        .collect::<Vec<&str>>();
                    folds.push((
                        raw_fold[1].parse().unwrap(),
                        if raw_fold[0] == "x" {
                            FoldAxis::X
                        } else {
                            FoldAxis::Y
                        },
                    ));
                }
                Err(_) => break,
            }
        }
        Origami { points, folds }
    }

    pub fn fold_once(&mut self) {
        let previous_points = self.points.clone();
            let fold = self.folds.drain(0..1).next().unwrap();
            for x in previous_points.iter() {
                let point = self.points.take(&x).unwrap();
                self.points
                    .insert(point.fold(fold.0, fold.1));
            }
    }

    pub fn fold_completely(&mut self)
    {
        while !self.folds.is_empty() {
            self.fold_once();
        }
    }

    pub fn display(&self) {
        let x_max = self.points.iter().map(|p| p.get_x()).max().unwrap();
        let y_max = self.points.iter().map(|p| p.get_y()).max().unwrap();
        for y in 0..=y_max {
            let mut line = String::new();
            for x in 0..=x_max {
                if self.points.contains(&Point::new(x, y)) {
                    line.push('#');
                }
                else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
    }

    pub fn count(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Origami;
    #[test]
    fn test_one_fold() {
        let mut origami = Origami::from_file("inputs_test.txt");
        origami.fold_once();
        assert_eq!(origami.count(), 17);
    }
    
    #[test]
    fn test_fold_completely() {
        let mut origami = Origami::from_file("inputs_test.txt");
        origami.fold_completely();
        assert_eq!(origami.count(), 16);
    }
    
    #[test]
    fn test_display() {
        let mut origami = Origami::from_file("inputs_test.txt");
        origami.fold_completely();
        origami.display();
    }
}

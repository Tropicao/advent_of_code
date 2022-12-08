pub struct Basin {
    holes: Vec<Vec<u8>>,
}

impl Basin {
    pub fn new(holes: &Vec<Vec<u8>>) -> Basin {
        Basin {
            holes: holes.clone(),
        }
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if y > 0 && self.holes[y - 1][x] > self.holes[y][x] {
            result.push((x, y - 1));
        }
        if y < self.holes.len() - 1 && self.holes[y + 1][x] > self.holes[y][x] {
            result.push((x, y + 1));
        }
        if x > 0 && self.holes[y][x - 1] > self.holes[y][x] {
            result.push((x - 1, y));
        }
        if x < self.holes[y].len() - 1 && self.holes[y][x + 1] > self.holes[y][x] {
            result.push((x + 1, y));
        }
        result
    }

    pub fn get_neighbors_in_basin(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![(x, y)];
        let neighbors_in_basin: Vec<(usize, usize)> = self
            .get_neighbors(x, y)
            .into_iter()
            .filter(|(i, j)| self.holes[*j][*i] < 9 && self.holes[*j][*i] > self.holes[y][x])
            .collect();
        for (x_h, y_h) in neighbors_in_basin {
            let mut next = self.get_neighbors_in_basin(x_h, y_h).into_iter().filter(|(i, j)| !result.contains(&(*i, *j))).collect();
            result.append(&mut next);
        }
        result
    }

    pub fn get_holes_in_basin(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.get_neighbors_in_basin(x, y)
    }

    pub fn is_on_horizontal_border(&self, y: usize) -> bool {
        y == 0 || y == self.holes.len() - 1
    }

    pub fn is_on_vertical_border(&self, x: usize) -> bool {
        x == 0 || x == self.holes[0].len() - 1
    }

    pub fn is_on_boarder(&self, x: usize, y: usize) -> bool {
        self.is_on_horizontal_border(y) ^ self.is_on_vertical_border(x)
    }
    pub fn is_on_corner(&self, x: usize, y: usize) -> bool {
        self.is_on_horizontal_border(y) && self.is_on_vertical_border(x)
    }

    pub fn get_low_points(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (j, line) in self.holes.iter().enumerate() {
            for (i, _) in line.iter().enumerate() {
                let neighbors = self.get_neighbors(i, j);
                if neighbors.len() == 4
                    || (self.is_on_boarder(i, j) && neighbors.len() == 3)
                    || (self.is_on_corner(i, j) && neighbors.len() == 2)
                {
                    result.push((i, j))
                }
            }
        }
        result
    }

    pub fn get_risk_levels_sum(&self) -> u32 {
        self.get_low_points()
            .iter()
            .map(|(x, y)| (self.holes[*y][*x] + 1) as u32)
            .sum()
    }

    pub fn get_basins_sizes(&self) -> Vec<usize> {
        self.get_low_points().into_iter().map(|(x, y)| self.get_holes_in_basin(x, y).len()).collect()
    }

    pub fn get_product_of_three_largest(&self) -> usize {
        let mut sizes = self.get_basins_sizes();
        sizes.sort();
        sizes.reverse();
        sizes.truncate(3);
        sizes.into_iter().fold(1, |acc, x|acc*x)
    }
}

#[cfg(test)]
mod tests {
    use super::Basin;
    use std::fmt::Debug;

    fn assert_vec_eq<T>(a: Vec<T>, b: Vec<T>)
    where T:PartialEq + Debug {
        if !(a.iter().all(|x| b.contains(x)) && b.iter().all(|x| a.contains(x))) {
            panic!("\nResult : {:?}\nExpected : {:?}", a, b);
        }
    }
    #[test]
    fn test_new_basin() {
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(Basin::new(&expected).holes, expected)
    }

    #[test]
    fn test_get_low_points() {
        let holes = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(
            Basin::new(&holes).get_low_points(),
            vec![(1, 0), (9, 0), (2, 2), (6, 4)]
        );
    }

    #[test]
    fn test_get_risk_levels_sum() {
        let holes = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(Basin::new(&holes).get_risk_levels_sum(), 15);
    }

    #[test]
    fn test_get_holes_in_bassin() {
        let holes = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let basin = Basin::new(&holes);
        assert_vec_eq(basin.get_holes_in_basin(1, 0), vec![(0, 0), (1, 0), (0, 1)]);
        assert_vec_eq(
            basin.get_holes_in_basin(9, 0),
            vec![
                (9, 0),
                (9, 1),
                (9, 2),
                (8, 0),
                (8, 1),
                (7, 0),
                (6, 0),
                (6, 1),
                (5, 0),
            ],
        );
        assert_vec_eq(
            basin.get_holes_in_basin(2, 2),
            vec![
                (0, 3),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3),
                (4, 1),
                (4, 2),
                (4, 3),
                (5, 2),
            ],
        );
        assert_vec_eq(
            basin.get_holes_in_basin(6, 4),
            vec![
                (5, 4),
                (6, 3),
                (6, 4),
                (7, 2),
                (7, 3),
                (7, 4),
                (8, 3),
                (8, 4),
                (9, 4),
            ],
        );
    }

    #[test]
    fn test_get_basins_size() {
        let holes = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let basin = Basin::new(&holes);
        assert_vec_eq(basin.get_basins_sizes(), vec![3, 9, 14, 9])
    }
    
    #[test]
    fn test_get_product_of_three_largest() {
        let holes = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let basin = Basin::new(&holes);
        assert_eq!(basin.get_product_of_three_largest(), 1134)

    }
}

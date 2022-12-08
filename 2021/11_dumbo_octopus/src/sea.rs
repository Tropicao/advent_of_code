pub struct Sea {
    octo: Vec<Vec<usize>>,
}

impl Sea {
    pub fn new(octo: &Vec<Vec<usize>>) -> Self {
        Sea { octo: octo.clone() }
    }

    fn increase_energy(&mut self) {
        for line in self.octo.iter_mut() {
            for x in line.iter_mut() {
                *x += 1;
            }
        }
    }

    fn get_flashing_octo(&self) -> Option<(usize, usize)> {
        for (y, l) in self.octo.iter().enumerate() {
            for (x, _) in l.iter().enumerate() {
                if self.octo[y][x] == 10 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn flash(&mut self, x: usize, y: usize) {
        self.octo[y][x] += 1;
        if y > 0 {
            if self.octo[y - 1][x] < 10 {
                self.octo[y - 1][x] += 1;
            }
            if x > 0 && self.octo[y - 1][x - 1] < 10 {
                self.octo[y - 1][x - 1] += 1;
            }
            if x < self.octo[y].len() - 1 && self.octo[y - 1][x + 1] < 10 {
                self.octo[y - 1][x + 1] += 1;
            }
        }
        if y < self.octo.len() - 1 {
            if self.octo[y + 1][x] < 10 {
                self.octo[y + 1][x] += 1;
            }
            if x > 0 && self.octo[y + 1][x - 1] < 10 {
                self.octo[y + 1][x - 1] += 1;
            }
            if x < self.octo[y].len() - 1 && self.octo[y + 1][x + 1] < 10 {
                self.octo[y + 1][x + 1] += 1;
            }
        }
        if x > 0 && self.octo[y][x - 1] < 10 {
            self.octo[y][x - 1] += 1
        }
        if x < self.octo[y].len() - 1 && self.octo[y][x + 1] < 10 {
            self.octo[y][x + 1] += 1
        }
    }

    fn reset_flashed_octopuses(&mut self) {
        for octo in self.octo.iter_mut().flatten() {
            if *octo >= 10 {
                *octo = 0;
            }
        }
    }

    pub fn run_step(&mut self) -> usize {
        let mut flashes = 0;
        self.increase_energy();

        // Make octopuses flash :
        // * flash all octo to 9, increase them, increase neigh
        // * repeat while there are 9s on grid
        loop {
            if let Some((x, y)) = self.get_flashing_octo() {
                self.flash(x, y);
                flashes +=1;
            } else {
                break;
            }
        }

        // All octopuses > 9 must be reset to 0
        self.reset_flashed_octopuses();
        flashes
    }

    pub fn run_x_steps(&mut self, x: usize) {
        for _ in 0..x {
            self.run_step();
        }
    }

    pub fn count_flashes_after(&mut self, x:usize) -> usize {
        (0..x).map(|_| self.run_step()).sum()
    }

    pub fn sync_flash_step(&mut self) -> usize {
        let nb_of_octo = self.octo.len() * self.octo[0].len();
        let mut current_step = 1;
        loop {
            let result = self.run_step();
            if result == nb_of_octo {
                return current_step;
            }
            current_step +=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sea;
    #[test]
    fn test_new_sea() {
        let data = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(Sea::new(&data).octo, data);
    }

    #[test]
    fn test_one_step() {
        let data = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];
        let mut sea = Sea::new(&data);
        sea.run_step();
        assert_eq!(sea.octo, expected);
    }

    #[test]
    fn test_two_step() {
        let data = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected = vec![
            vec![4, 5, 6, 5, 4],
            vec![5, 1, 1, 1, 5],
            vec![6, 1, 1, 1, 6],
            vec![5, 1, 1, 1, 5],
            vec![4, 5, 6, 5, 4],
        ];
        let mut sea = Sea::new(&data);
        sea.run_x_steps(2);
        assert_eq!(sea.octo, expected);
    }

    #[test]
    pub fn test_one_step_large_grid() {
        let data = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let expected = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];
        let mut sea = Sea::new(&data);
        sea.run_x_steps(1);
        assert_eq!(sea.octo, expected);
    }

    #[test]
    pub fn test_ten_step_large_grid() {
        let data = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let expected = vec![
            vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
            vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
            vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
            vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
            vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
            vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
            vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
            vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
            vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
            vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0]
        ];
        let mut sea = Sea::new(&data);
        sea.run_x_steps(10);
        assert_eq!(sea.octo, expected);
    }

    #[test]
    pub fn test_count_flashes_ten() {
        let data = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let mut sea = Sea::new(&data);
        assert_eq!(sea.count_flashes_after(10), 204);
    }

    #[test]
    pub fn test_count_flashes_hundred() {
        let data = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let mut sea = Sea::new(&data);
        assert_eq!(sea.count_flashes_after(100), 1656);
    }

    #[test]
    fn test_get_sync_flash_step() {
        let data = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let mut sea = Sea::new(&data);
        assert_eq!(sea.sync_flash_step(), 195);
    }
}

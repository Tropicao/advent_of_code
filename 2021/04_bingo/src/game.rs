mod grid;
use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub struct Game {
    draws: Vec<u32>,
    grids: Vec<Grid>,
}

impl Game {
    fn parse_draws(line: &str) -> Vec<u32> {
        let mut result: Vec<u32> = line
            .trim()
            .split(',')
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|x| panic!("Cannot convert : {}", x))
            })
            .collect();
        result.reverse();
        result
    }

    fn parse_grids(reader: &mut BufReader<File>) -> Vec<Grid> {
        let mut result = Vec::new();
        let mut temp = String::new();
        let mut current_grid: Vec<Vec<u32>> = Vec::new();
        loop {
            match reader.read_line(&mut temp) {
                Ok(0) => {
                    if !current_grid.is_empty() {
                        result.push(Grid::new(current_grid));
                    }
                    break;
                }
                Ok(1) => {
                    result.push(Grid::new(current_grid.clone()));
                    current_grid.clear()
                }
                _ => current_grid.push(
                    temp.trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect(),
                ),
            };
            temp.clear();
        }
        result
    }

    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut temp = String::new();
        reader
            .read_line(&mut temp)
            .expect("Error reading draws line");
        let draws = Game::parse_draws(&temp);
        //Consume empty line
        reader
            .read_line(&mut temp)
            .expect("Error consuming empty line before grids");
        let grids = Game::parse_grids(&mut reader);
        Game { draws, grids }
    }

    pub fn draw(&mut self) -> u32 {
        let value = self.draws.pop().unwrap();
        for g in self.grids.iter_mut() {
            g.draw(value);
        }
        value
    }

    pub fn has_a_winning_grid(&self) -> Option<usize> {
        for (index, grid) in self.grids.iter().enumerate() {
            if grid.is_winning() {
                return Some(index);
            }
        }
        None
    }

    pub fn get_score(&self, winning_grid: usize, last_draw: u32) -> u32 {
        self.grids[winning_grid].get_unmarked_sum()*last_draw
    }

    pub fn remove_grid(&mut self, index:usize) {
        self.grids.remove(index);
    }

    pub fn count_grids(&self) -> usize {
        self.grids.len()
    }
}

#[cfg(test)]
mod test {
    use super::{Game, Grid};
    #[test]
    fn test_load_draws() {
        let mut draws = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        draws.reverse();
        let game = Game::new("inputs_test.txt");
        assert_eq!(game.draws, draws);
    }

    #[test]
    fn test_load_grids() {
        let grids = vec![
            Grid::new(vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Grid::new(vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Grid::new(vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let game = Game::new("inputs_test.txt");
        assert_eq!(game.grids, grids);
    }

    #[test]
    fn test_draw_one() {
        let mut game = Game::new("inputs_test.txt");
        game.draw();
        // No winning draw yet
        assert_eq!(game.has_a_winning_grid(), None);
    }

    #[test]
    fn test_draw_eleven() {
        let mut game = Game::new("inputs_test.txt");
        for _ in 0..11 {
            game.draw();
        }
        // No winning draw yet
        assert_eq!(game.has_a_winning_grid(), None);
    }

    #[test]
    fn test_draw_twelve() {
        let mut game = Game::new("inputs_test.txt");
        for _ in 0..12 {
            game.draw();
        }
        // Third board must be winning
        assert_eq!(game.has_a_winning_grid(), Some(2));
    }

    #[test]
    fn test_get_score() {
        let mut game = Game::new("inputs_test.txt");
        for _ in 0..12 {
            game.draw();
        }
        // Third board must be winning
        assert_eq!(game.get_score(2, 24), 4512);
    }

    #[test]
    fn test_remove_grid() {
        let grids = vec![
            Grid::new(vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Grid::new(vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let mut game = Game::new("inputs_test.txt");
        game.remove_grid(1);
        assert_eq!(game.grids, grids);
    }
    
    #[test]
    fn test_loosing_score()
    {
        let mut game = Game::new("inputs_test.txt");
        let mut last_draw = 0;
        while game.count_grids() > 1 {
            while game.has_a_winning_grid().is_none()
            {
                game.draw();
            }
            game.remove_grid(game.has_a_winning_grid().unwrap());
        }
        
        while game.has_a_winning_grid().is_none()
        {
            last_draw = game.draw();
        }
        assert_eq!(game.get_score(game.has_a_winning_grid().unwrap(), last_draw), 1924); 
    }
}

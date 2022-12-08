mod point;
use point::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<Point>>;
pub struct Maze {
    chitons: Vec<Vec<usize>>,
    chitons_big: Vec<Vec<usize>>,
}

impl Maze {
    pub fn from_file(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut chitons = vec![];
        for line in reader.lines() {
            chitons.push(
                line.unwrap()
                    .trim()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>(),
            );
        }
        let chitons_big = Maze::build_big_map(&chitons);
        Maze { chitons, chitons_big }
    }

    fn build_big_map(chunk : &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result:Vec<Vec<usize>> = vec![];
        // Build first row of chunks
        for line in chunk {
            let mut new_line = vec![];
            new_line.push(line.clone());
            let mut increased_line = line.clone();
            for _ in 0..4 {
                increased_line = increased_line.iter().map(|&x| if x < 9 {x + 1} else { 1 }).collect::<Vec<usize>>();
                new_line.push(increased_line.clone());
            }
            result.push(new_line.into_iter().flatten().collect::<Vec<usize>>());
        }

        // Next rows are built from previous row
        for i in chunk.len()..chunk.len()*5 {
            let new_line = result[i-chunk.len()].iter().map(|&x| if x < 9 {x + 1} else { 1 }).collect::<Vec<usize>>();
            result.push(new_line);
        }
        result
    }

    fn get_next_candidate(&self, map: &Map) -> (usize, usize)
    {
        let mut current_min = usize::MAX;
        let mut x_result = 0;
        let mut y_result = 0;
        for (y, line) in map.iter().enumerate() {
            for(x, column) in line.iter().enumerate() {
                if !column.selected() && column.cost() < current_min {
                    current_min = column.cost();
                    x_result = x;
                    y_result = y;
                }
            }
        }
        (y_result, x_result)
    }

    fn update_neighbours(chitons: &Vec<Vec<usize>>, map: &mut Map, local_cost: usize, y: usize, x: usize) {
        // Assume we can only go down, left or right
        if y < map.len() - 1 && !map[y+1][x].selected(){
            if local_cost + chitons[y+1][x] < map[y+1][x].cost() {
                map[y+1][x].set_cost(local_cost + chitons[y+1][x])
            }
        }
        if x < map.len() - 1 && !map[y][x+1].selected(){
            if local_cost + chitons[y][x+1] < map[y][x+1].cost() {
                map[y][x+1].set_cost(local_cost + chitons[y][x+1])
            }
        }
        if x > 0 && !map[y][x-1].selected(){
            if local_cost + chitons[y][x-1] < map[y][x-1].cost() {
                map[y][x-1].set_cost(local_cost + chitons[y][x-1])
            }
        }
        if y > 0 && !map[y-1][x].selected(){
            if local_cost + chitons[y-1][x] < map[y-1][x].cost() {
                map[y-1][x].set_cost(local_cost + chitons[y-1][x])
            }
        }
    }

    fn find_shortest_path(&self, original_map: &Vec<Vec<usize>>) -> usize{
        let mut map  =  vec![];
        let map_len = original_map.len();

        if map_len == 0 {
            return 0
        }
        
        // Initialize map
        for _ in 0..map_len {
            map.push(vec![Point::new(usize::MAX); map_len])
        }
        map[0][0].set_cost( 0);

        // Start research
        loop 
        {
            let(y, x) = self.get_next_candidate(&map);
            map[y][x].select();
            if x == map_len - 1 && y == map_len-1 { break}
            let local_cost = map[y][x].cost();
            Maze::update_neighbours(&original_map, &mut map, local_cost, y, x);
        }
        
        map[map_len-1][map_len-1].cost()
    }

    pub fn get_little_cave_path(&self) -> usize {
        self.find_shortest_path(&self.chitons)
    }

    pub fn get_big_cave_path(&self) -> usize {
        self.find_shortest_path(&self.chitons_big)
    }
}

#[cfg(test)]
mod tests {
    use super::Maze;
    #[test]
    fn test_load_chitons() {
        let expected_chitons = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];
        let maze = Maze::from_file("inputs_test.txt");
        assert_eq!(maze.chitons, expected_chitons);
    }
    
    #[test]
    fn test_get_shortest_path() {
        let maze = Maze::from_file("inputs_test.txt");
        assert_eq!(maze.get_little_cave_path(), 40)
    }
    
    #[test]
    fn test_get_shortest_path_in_big_cave() {
        let maze = Maze::from_file("inputs_test.txt");
        assert_eq!(maze.get_big_cave_path(), 315)
    }
}

struct Tree {
    size: usize,
    seen: bool,
}

impl From<usize> for Tree {
    fn from(size: usize) -> Self {
        Tree { size, seen: false }
    }
}
struct Forest {
    grid: Vec<Vec<Tree>>,
    size: usize,
}

impl Forest {
    fn new(input: &str) -> Self {
        let mut result = Forest {
            grid: vec![],
            size: input.lines().next().unwrap().trim().len(),
        };
        for line in input.lines() {
            result.grid.push(
                line.trim()
                    .chars()
                    .map(|x| Tree::from(x.to_digit(10).unwrap() as usize))
                    .collect::<Vec<Tree>>(),
            )
        }
        result
    }

    fn count_visible_trees(&mut self) -> usize {
        //Left to right and right to left
        for line in self.grid.iter_mut() {
            let mut max = None;
            for t in line.iter_mut() {
                if max.is_none() || t.size > max.unwrap() {
                    max = Some(t.size);
                    t.seen = true;
                }
            }
            max = None;
            for t in line.iter_mut().rev() {
                if max.is_none() || t.size > max.unwrap() {
                    max = Some(t.size);
                    t.seen = true;
                }
            }
        }
        //Top to bottom
        for column in 0..self.size {
            let mut max = None;
            for line in 0..self.size {
                if max.is_none() || self.grid[line][column].size > max.unwrap() {
                    max = Some(self.grid[line][column].size);
                    self.grid[line][column].seen = true;
                }
            }
            max = None;
            for line in (0..self.size).rev() {
                if max.is_none() || self.grid[line][column].size > max.unwrap() {
                    max = Some(self.grid[line][column].size);
                    self.grid[line][column].seen = true;
                }
            }
        }
        self.grid.iter().flatten().filter(|&x| x.seen).count()
    }

    fn scenic_score(&self, line:usize, column: usize) -> usize {
        let mut result = 1;
        // Left
        if column > 0 {
            let mut lower_trees = (0..column).rev().take_while(|&x| self.grid[line][x].size < self.grid[line][column].size).count() + 1;
            if column + 1 - lower_trees == 0 {
                lower_trees -=1;
            }
            result *= lower_trees;
        }
        // Right
        if column < self.size -1 {
            let mut lower_trees = (column+1..self.size).take_while(|&x| self.grid[line][x].size < self.grid[line][column].size).count() + 1;
            if self.size - column - lower_trees == 0 {
                lower_trees -=1;
            }
            result *= lower_trees;
        }
        // Top
        if line > 0 {
            let mut lower_trees = (0..line).rev().take_while(|&x| self.grid[x][column].size < self.grid[line][column].size).count() + 1;
            if line + 1 - lower_trees == 0 {
                lower_trees -=1;
            }
            result *= lower_trees;
        }
        // Bottom
        if line < self.size - 1 {
            let mut lower_trees = (line + 1..self.size).take_while(|&x| self.grid[x][column].size < self.grid[line][column].size).count() + 1;
            if self.size - line - lower_trees == 0 {
                lower_trees -=1;
            }
            result *= lower_trees;
        }
        result
    }
}



pub fn part_1(input: &str) -> usize {
    let mut forest = Forest::new(input);
    forest.count_visible_trees()
}

pub fn part_2(input: &str) -> usize {
    let forest = Forest::new(input);
    let mut scenic_scores = vec![];
    for i in 1..forest.size-1 {
        for j in 1..forest.size-1 {
            scenic_scores.push(forest.scenic_score(i, j));
        }
    }
    *scenic_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Set test input in this variable
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 21)
    }

    #[test]
    fn test_scenic_view()
    {
        let forest = Forest::new(TEST_INPUT);
        assert_eq!(forest.scenic_score(1, 2), 4);
    }

    #[test]
    fn test_scenic_view_bis()
    {
        let forest = Forest::new(TEST_INPUT);
        assert_eq!(forest.scenic_score(3, 2), 8);
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 8)
    }
}

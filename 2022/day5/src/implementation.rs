struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}
struct Crates {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        // Input line has the form :
        // "move X from Y to Z\n"
        let mut raw_data = input.split_ascii_whitespace().skip(1).step_by(2);
        Instruction {
            count: raw_data.next().unwrap().parse::<usize>().unwrap(),
            src: raw_data.next().unwrap().parse::<usize>().unwrap(),
            dst: raw_data.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

impl Crates {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let mut line = lines.next();
        let nb_stacks = (line.unwrap().len() + 1) / 4;
        let mut result = Crates {
            stacks: vec![vec![]; nb_stacks],
            instructions: vec![]
        };
        'parsing_stacks: while line.is_some() {
            for (index, char) in line.unwrap().chars().skip(1).step_by(4).enumerate() {
                if char.is_alphabetic() {
                    result.stacks[index].push(char);
                } else if char.is_ascii_digit() {
                    // We have reached the crates indexes line
                    break 'parsing_stacks;
                }
            }
            line = lines.next();
        }
        // Skip crates indexes line
        lines.next();
        // Skip empty line
        line = lines.next();

        // Parse moving instructions
        while line.is_some() {
            result.instructions.push(Instruction::new(line.unwrap()));
            line = lines.next();
        }
        result
    }

    pub fn execute_instructions(&mut self) {
        for instruction in self.instructions.iter() {
            for _ in 0..instruction.count {
                let element = self.stacks[instruction.src-1].remove(0);
                self.stacks[instruction.dst-1].insert(0, element);
            }
        }
    }

    pub fn execute_instructions_with_better_crane(&mut self) {
        for instruction in self.instructions.iter() {
            let mut crates_to_move = self.stacks[instruction.src-1].drain(..instruction.count).collect::<Vec<char>>();
            crates_to_move.append(&mut self.stacks[instruction.dst-1]);
            self.stacks[instruction.dst-1] = crates_to_move;
        }
    }

    pub fn get_top_elements(&self) -> String {
        self.stacks.iter().fold(String::new(), |acc, x| {acc + &String::from(x[0])})
    }
}

pub fn part_1(input: &str) -> String {
    let mut supply = Crates::new(input);
    supply.execute_instructions();
    supply.get_top_elements()
}

pub fn part_2(input: &str) -> String {
    let mut supply = Crates::new(input);
    supply.execute_instructions_with_better_crane();
    supply.get_top_elements()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Set test input in this variable
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), "CMZ")
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), "MCD")
    }
}

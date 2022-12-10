enum Instruction {
    Add(isize),
    Noop,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value = value.split_ascii_whitespace().collect::<Vec<&str>>();
        match split_value[0] {
            "addx" => Ok(Instruction::Add(split_value[1].parse().unwrap())),
            "noop" => Ok(Instruction::Noop),
            _ => Err("Can not parse value into instruction"),
        }
    }
}

#[derive(Default)]
struct CPU {
    instructions: Vec<Instruction>,
    reg: isize,
    cycle: usize,
}

impl CPU {
    fn new(input: &str) -> Self {
        CPU {
            instructions: input
                .lines()
                .map(|x| Instruction::try_from(x.trim()).unwrap())
                .collect(),
            reg: 1,
            ..Default::default()
        }
    }

    fn signal_strength_during_cycle(&mut self, cycle: usize) -> isize {
        let mut current_instruction_index = 0;
        self.reg = 1;
        self.cycle = 0;
        while self.cycle < cycle {
            match self.instructions[current_instruction_index] {
                Instruction::Add(x) => {
                    self.cycle += 1;
                    if self.cycle + 1 < cycle {
                        self.cycle += 1;
                        self.reg += x;
                    }
                }
                Instruction::Noop => self.cycle += 1,
            }
            current_instruction_index += 1;
        }
        self.reg * self.cycle as isize
    }

    fn get_symbols(&self) -> String {
        let mut result = String::new();
        if self.cycle != 0 && self.cycle % 40 == 0 {
            result.push('\n');
        }

        result.push(
            if (self.reg - 1..self.reg + 2).contains(&((self.cycle % 40) as isize)) {
                '#'
            } else {
                '.'
            },
        );

        result
    }

    fn crt(&mut self) -> String {
        self.reg = 1;
        self.cycle = 0;
        let mut result = String::new();
        for i in self.instructions.iter() {
            match i {
                Instruction::Add(x) => {
                    result += &self.get_symbols();
                    self.cycle += 1;

                    result += &self.get_symbols();
                    self.cycle += 1;
                    self.reg += x;
                }
                Instruction::Noop => {
                    result += &self.get_symbols();
                    self.cycle += 1;
                }
            }
        }
        String::from(result.trim())
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cpu = CPU::new(input);
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|x| cpu.signal_strength_during_cycle(*x) as usize)
        .sum()
}

pub fn part_2(input: &str) -> String {
    let mut cpu = CPU::new(input);
    cpu.crt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    // Set test input in this variable
    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    const TEST_2_OUTPUT: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test_case(TEST_INPUT, 20, 420; "signal strength_at_cycle_20")]
    #[test_case(TEST_INPUT, 60, 1140; "signal strength_at_cycle_60")]
    #[test_case(TEST_INPUT, 100, 1800; "signal strength_at_cycle_100")]
    #[test_case(TEST_INPUT, 140, 2940; "signal strength_at_cycle_140")]
    #[test_case(TEST_INPUT, 180, 2880; "signal strength_at_cycle_180")]
    #[test_case(TEST_INPUT, 220, 3960; "signal strength_at_cycle_220")]
    fn test_signal_at_specific_cycle(input: &str, cycle: usize, output: isize) {
        let mut cpu = CPU::new(input);
        assert_eq!(cpu.signal_strength_during_cycle(cycle), output);
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 13140)
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), TEST_2_OUTPUT)
    }
}

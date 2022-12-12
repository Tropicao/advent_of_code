use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Default, Debug)]
enum Operator {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Operand {
    Digit(u64),
    Old,
}

impl Default for Operand {
    fn default() -> Self {
        Self::Digit(0)
    }
}

#[derive(Default)]
struct Monkey {
    remainders: Vec<u64>,
    operator: Operator,
    operand: Operand,
    test_operand: u64,
    target_true: usize,
    target_false: usize,
    inspections_counter: u64,
}

fn parse_remainders(line: &str) -> IResult<&str, Vec<u64>> {
    let (raw_list, _) = tag("  Starting items: ")(line)?;
    separated_list1(tag(", "), map(digit1, |x: &str| x.parse::<u64>().unwrap()))(raw_list)
}

fn parse_operation(line: &str) -> IResult<&str, (Operator, Operand)> {
    let filter_operator = |x: &str| match x {
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "-" => Operator::Sub,
        "+" => Operator::Add,
        _ => panic!("Invalid operand {x}"),
    };

    let match_operand = |x: &str| match x {
        "old" => Operand::Old,
        _ => Operand::Digit(x.parse::<u64>().unwrap()),
    };
    let result = preceded(
        tag("Operation: new = old"),
        tuple((
            preceded(
                tag(" "),
                map(
                    alt((tag("*"), tag("/"), tag("+"), tag("-"))),
                    filter_operator,
                ),
            ),
            preceded(tag(" "), map(alt((tag("old"), digit1)), match_operand)),
        )),
    )(line.trim());
    result
}

fn parse_test_operand(line: &str) -> IResult<&str, u64> {
    preceded(
        tag("Test: divisible by "),
        map(digit1, |x: &str| x.parse::<u64>().unwrap()),
    )(line.trim())
}

fn parse_action(line: &str, switch: bool) -> IResult<&str, usize> {
    let prefix = match switch {
        true => "If true: throw to monkey ",
        false => "If false: throw to monkey ",
    };
    preceded(
        tag(prefix),
        map(digit1, |x: &str| x.parse::<usize>().unwrap()),
    )(line.trim())
}

impl Monkey {
    fn new(input: &str) -> Self {
        let mut monkey = Monkey {
            ..Default::default()
        };
        let mut lines = input.lines();
        // Skip monkey id line
        lines.next();
        monkey.remainders = parse_remainders(lines.next().unwrap()).finish().unwrap().1;
        (monkey.operator, monkey.operand) =
            parse_operation(lines.next().unwrap()).finish().unwrap().1;
        monkey.test_operand = parse_test_operand(lines.next().unwrap())
            .finish()
            .unwrap()
            .1;
        monkey.target_true = parse_action(lines.next().unwrap(), true)
            .finish()
            .unwrap()
            .1;
        monkey.target_false = parse_action(lines.next().unwrap(), false)
            .finish()
            .unwrap()
            .1;
        monkey
    }

    fn has_items(&self) -> bool {
        !self.remainders.is_empty()
    }

    fn inspect(&mut self) {
        self.inspections_counter += 1;
        let operand = if let Operand::Digit(x) = self.operand {
            x as u64
        } else {
            self.remainders[0]
        };
        match self.operator {
            Operator::Add => self.remainders[0] += operand,
            Operator::Sub => self.remainders[0] -= operand,
            Operator::Mul => self.remainders[0] *= operand,
            Operator::Div => self.remainders[0] /= operand,
        }
    }

    fn loose_interest(&mut self) {
        self.remainders[0] = self.remainders[0].div_euclid(3);
    }

    fn get_target(&self) -> usize {
        if let Some(0) = self.remainders[0].checked_rem_euclid(self.test_operand as u64) {
            self.target_true
        } else {
            self.target_false
        }
    }
}

#[derive(Default)]
struct Game {
    monkeys: Vec<Monkey>,
    divisor: u64,
}

impl Game {
    fn new(input: &str) -> Self {
        let mut result = Game {
            ..Default::default()
        };
        for raw_monkey in input.split("\n\n") {
            result.monkeys.push(Monkey::new(raw_monkey));
        }
        result.divisor = result.monkeys.iter().map(|x| x.test_operand).product();
        result
    }

    fn run_monkey_fast(&mut self, index: usize) {
        while self.monkeys[index].has_items() {
            self.monkeys[index].inspections_counter += 1;
            let mut value = self.monkeys[index].remainders.remove(0);
            let test_operand = self.monkeys[index].test_operand;
            let operand = if let Operand::Digit(x) = self.monkeys[index].operand {
                x
            } else {
                value
            };
            value %= self.divisor;
            value = match self.monkeys[index].operator {
                Operator::Add => value + operand,
                Operator::Mul => value * operand,
                _ => panic!("Unsupported operand"),
            };
            let target = if value % test_operand == 0 {
                self.monkeys[index].target_true
            } else {
                self.monkeys[index].target_false
            };
            self.monkeys[target].remainders.push(value);
        }
    }

    fn run_monkey(&mut self, index: usize) {
        while self.monkeys[index].has_items() {
            self.monkeys[index].inspect();
            self.monkeys[index].loose_interest();
            let target = self.monkeys[index].get_target();
            let value = self.monkeys[index].remainders.remove(0);
            self.monkeys[target].remainders.push(value)
        }
    }

    fn run_all_monkeys(&mut self, worry_div: bool) {
        if worry_div {
            for i in 0..self.monkeys.len() {
                self.run_monkey(i);
            }
        } else {
            for i in 0..self.monkeys.len() {
                self.run_monkey_fast(i);
            }
        }
    }

    fn run_cycles(&mut self, x: usize, worry_div: bool) {
        for _ in 0..x {
            self.run_all_monkeys(worry_div);
        }
    }

    fn get_monkey_businesses(&self) -> Vec<u64> {
        self.monkeys.iter().map(|x| x.inspections_counter).collect()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut game = Game::new(input);
    game.run_cycles(20, true);
    let mut businesses = game.get_monkey_businesses();
    businesses.sort();
    (businesses.pop().unwrap() * businesses.pop().unwrap()) as usize
}

pub fn part_2(input: &str) -> usize {
    let mut game = Game::new(input);
    game.run_cycles(10000, false);
    let mut businesses = game.get_monkey_businesses();
    businesses.sort();
    (businesses.pop().unwrap() * businesses.pop().unwrap()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    // Set test input in this variable
    const TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test_case(1, vec![4, 6, 0, 0]; "monkeys_items_after 1 round")]
    #[test_case(2, vec![5, 5, 0, 0]; "monkeys_items_after 2 round")]
    fn check_items_after_round(cycles: usize, expected: Vec<u64>) {
        let mut game = Game::new(TEST_INPUT);
        game.run_cycles(cycles, true);
        assert_eq!(
            game.monkeys
                .iter()
                .map(|x| x.remainders.len() as u64)
                .collect::<Vec<u64>>(),
            expected
        );
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 10605)
    }

    #[test_case(1, vec![2, 4, 3, 6]; "after 1 round")]
    #[test_case(20, vec![99, 97, 8, 103]; "after 20 round")]
    #[test_case(1000, vec![5204, 4792, 199, 5192]; "after 1000 round")]
    #[test_case(2000, vec![10419, 9577, 392, 10391]; "after 2000 round")]
    #[test_case(3000, vec![15638, 14358, 587, 15593]; "after 3000 round")]
    #[test_case(4000, vec![20858, 19138, 780, 20797]; "after 4000 round")]
    #[test_case(5000, vec![26075, 23921, 974, 26000]; "after 5000 round")]
    #[test_case(6000, vec![31294, 28702, 1165, 31204]; "after 6000 round")]
    #[test_case(7000, vec![36508, 33488, 1360, 36400]; "after 7000 round")]
    #[test_case(8000, vec![41728, 38268, 1553, 41606]; "after 8000 round")]
    #[test_case(9000, vec![46945, 43051, 1746, 46807]; "after 9000 round")]
    #[test_case(10000, vec![52166, 47830, 1938, 52013]; "after 10000 round")]
    fn test_businesses_after_round(rounds: usize, expected: Vec<u64>) {
        let mut game = Game::new(TEST_INPUT);
        game.run_cycles(rounds, false);
        assert_eq!(game.get_monkey_businesses(), expected)
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 2713310158)
    }
}

use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult,
};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Value {
    Integer(usize),
    List(Vec<Value>),
}

struct Values(Vec<Value>);

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            if i < self.0.len() - 1 {
                writeln!(f, "{v}")?;
            } else {
                write!(f, "{v}")?;
            }
        }
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        match self {
            Value::Integer(a) => result += &a.to_string(),
            Value::List(b) => {
                result.push('[');
                for (i, v) in b.iter().enumerate() {
                    if i != 0 {
                        result.push(',');
                    }
                    result += &format!("{}", v);
                }
                result.push(']');
            }
        };
        write!(f, "{result}")
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::Integer(a), Value::List(b)) => {
                Value::List(vec![Value::Integer(*a)]).cmp(&Value::List(b.clone()))
            }
            (Value::List(a), Value::Integer(b)) => {
                Value::List(a.clone()).cmp(&Value::List(vec![Value::Integer(*b)]))
            }
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Integer(0)
    }
}

impl TryFrom<&str> for Value {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match all_consuming(parse_list)(value).finish() {
            Ok((_, value)) => Ok(value),
            Err(err) => Err(format!("{err}")),
        }
    }
}

fn parse_integer(input: &str) -> IResult<&str, Value> {
    map(digit1, |x: &str| {
        Value::Integer(x.parse::<usize>().unwrap())
    })(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    // 1, 1
    let content_parser = many0(terminated(alt((parse_integer, parse_list)), opt(tag(","))));
    delimited(tag("["), map(content_parser, Value::List), tag("]"))(input)
    //alt((map(parse_list, |x| Value::List(x)), parse_integer))
}

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    let mut result = vec![];
    let mut lines = input.lines();
    while let (Some(first), Some(second)) = (lines.next(), lines.next()) {
        result.push((
            Value::try_from(first).unwrap(),
            Value::try_from(second).unwrap(),
        ));
        lines.next();
    }
    result
}

pub fn part_1(input: &str) -> usize {
    let value = parse_input(input);
    value
        .iter()
        .enumerate()
        .filter_map(|(index, (a, b))| if a < b { Some(index + 1) } else { None })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let divider_1 = Value::try_from("[[2]]").unwrap();
    let divider_2 = Value::try_from("[[6]]").unwrap();
    let mut values = Values(vec![]);
    values.0 = input
        .lines()
        .filter_map(|x| {
            if !x.trim().is_empty() {
                Some(Value::try_from(x).unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<Value>>();
    values.0.push(divider_1.clone());
    values.0.push(divider_2.clone());
    values.0.sort();
    values
        .0
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v == &divider_1 || v == &divider_2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    // Set test input in this variable
    const TEST_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    const TEST_OUTPUT_PART_2: &str = "\
[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]\
";

    #[test]
    fn test_simple_load() {
        // Adjust part 1 test to match provided challenge example
        let value = Value::try_from("[1,1,1,1]");
        assert_eq!(
            value,
            Ok(Value::List(vec![
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(1)
            ]))
        );
    }
    #[test]
    fn test_intermediate_load() {
        // Adjust part 1 test to match provided challenge example
        let value = Value::try_from("[[1,2],[3,4]]");
        assert_eq!(
            value,
            Ok(Value::List(vec![
                Value::List(vec![Value::Integer(1), Value::Integer(2)]),
                Value::List(vec![Value::Integer(3), Value::Integer(4)])
            ]))
        );
    }

    #[test]
    fn test_load_empty_list() {
        let value = Value::try_from("[]");
        assert_eq!(value, Ok(Value::List(vec![])));
    }

    #[test_case(1, true)]
    #[test_case(2, true)]
    #[test_case(3, false)]
    #[test_case(4, true)]
    #[test_case(5, false)]
    #[test_case(6, true)]
    #[test_case(7, false)]
    #[test_case(8, false)]
    fn test_pairs(input_index: usize, expected: bool) {
        let pairs = parse_input(TEST_INPUT);
        if expected {
            assert!(pairs[input_index - 1].0 < pairs[input_index - 1].1);
        } else {
            assert!(pairs[input_index - 1].0 > pairs[input_index - 1].1);
        }
    }

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 13)
    }

    #[test]
    fn test_print() {
        let value = Value::try_from("[[1,2],[3,4]]").unwrap();
        assert_eq!(format!("{value}"), "[[1,2],[3,4]]");
    }

    #[test]
    fn test_sort() {
        let mut values = Values(vec![]);
        values.0 = TEST_INPUT
            .lines()
            .filter_map(|x| {
                if !x.trim().is_empty() {
                    Some(Value::try_from(x).unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<Value>>();
        values.0.push(Value::try_from("[[2]]").unwrap());
        values.0.push(Value::try_from("[[6]]").unwrap());
        values.0.sort();
        assert_eq!(format!("{values}"), TEST_OUTPUT_PART_2);
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 140)
    }
}

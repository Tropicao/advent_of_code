use core::num::ParseIntError;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Number {
    Literal(u8),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    pub fn new(left: Number, right: Number) -> Self {
        Number::Pair(Box::new(left), Box::new(right))
    }

    pub fn convert_to_string(&self) -> String {
        let mut result = String::new();
        match self {
            Number::Literal(v) => result.push_str(&format!("{}", v)),
            Number::Pair(left, right) => {
                result.push('[');
                result.push_str(&left.convert_to_string());
                result.push(',');
                result.push_str(&right.convert_to_string());
                result.push(']');
            }
        }
        result
    }

    fn find_closing_bracket(s: &str) -> usize {
        let mut opened = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => opened += 1,
                ']' => match opened {
                    0 => return i,
                    _ => opened -= 1,
                },
                _ => continue,
            }
        }
        0
    }

    pub fn convert_from_string(s: &str) -> Result<Self, ParseIntError> {
        let mut index = 0;
        // First char is an opening bracket, skip it
        index += 1;
        let left = match s.chars().nth(index).unwrap() {
            '[' => {
                let closing_bracket_pos = index + 1 + Number::find_closing_bracket(&s[index + 1..]);
                let result = Number::convert_from_string(&s[index..=closing_bracket_pos])?;
                index += closing_bracket_pos;
                result
            }
            _ => {
                let comma_pos = s.find(",").unwrap();
                let result = Number::Literal((&s[index..comma_pos]).parse()?);
                index += comma_pos - 1;
                result
            }
        };
        // Next char is a comma, skip it
        index += 1;
        let right = match s.chars().nth(index).unwrap() {
            '[' => {
                let closing_bracket_pos = index + 1 + Number::find_closing_bracket(&s[index + 1..]);
                let result = Number::convert_from_string(&s[index..=closing_bracket_pos])?;
                index += closing_bracket_pos;
                result
            }
            _ => {
                let closing_bracket_pos = index + Number::find_closing_bracket(&s[index..]);
                Number::Literal((&s[index..closing_bracket_pos]).parse()?)
            }
        };
        Ok(Number::Pair(Box::new(left), Box::new(right)))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    pub fn is_pair(&self) -> bool {
        !self.is_literal()
    }

    fn explode_recur(self, stage: usize) -> (Self, Option<(u8, u8)>) {
        println!("Dealing with {}", self.to_string());
        let mut child_exploded = false;
        if stage == 4 && self.is_pair() {
            if let Number::Pair(l, r) = self {
                // Last stage is assumed to be composed only of literals
                return (Number::Literal(0), Some((l.unwrap(), r.unwrap())));
            }
        }

        if self.is_literal() {
            return (self, Some((0, 0)));
        }

        if let Number::Pair(l, r) = self {
            if let Number::Pair(_, _) = *l {
                let (new_l, inc) = l.explode_recur(stage+1);
                if let Some((inc_l, inc_r)) = inc {
                    child_exploded = true;
                    if let Number::Literal(r_val) = *r {
                        return (Number::new(new_l, Number::Literal(r_val + inc_r)), Some((inc_l, 0)));           
                    }
                    else {
                        return (Number::new(new_l, *r), Some((inc_l, inc_r)));           
                    }
                }
            }
            if !child_exploded {
                if let Number::Pair(_, _) = *r {
                    let (new_r, inc) = r.explode_recur(stage+1);
                    if let Some((inc_l, inc_r)) = inc {
                        child_exploded = true;
                        if let Number::Literal(l_val) = *l {
                            return (Number::new(Number::Literal(l_val + inc_l), new_r), Some((0, inc_r)));           
                        }
                        else {
                            return (Number::new(*l, new_r), Some((inc_l, inc_r)));           
                        }
                    }
                }
            }
        }



        (Number::Literal(0), Some((0, 0)))
    }

    fn explode(self) -> Self {
        let (new_num,_) = self.explode_recur(0);
        new_num
    }

    fn unwrap(self) -> u8 {
        match self {
            Number::Literal(val) => val,
            _ => panic!("called `Number::unwrap()` on a `Pair` value"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.convert_to_string())
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Number::convert_from_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use std::str::FromStr;
    #[test]
    fn test_new() {
        let expected = Number::Pair(Box::new(Number::Literal(1)), Box::new(Number::Literal(2)));
        assert_eq!(
            Number::new(Number::Literal(1), Number::Literal(2)),
            expected
        );
    }
    #[test]
    fn test_to_string() {
        assert_eq!(
            Number::new(Number::Literal(1), Number::Literal(2)).to_string(),
            "[1,2]"
        );
    }
    #[test]
    fn test_to_string_advanced() {
        assert_eq!(
            Number::new(
                Number::Pair(Box::new(Number::Literal(1)), Box::new(Number::Literal(2))),
                Number::Pair(Box::new(Number::Literal(3)), Box::new(Number::Literal(4)))
            )
            .to_string(),
            "[[1,2],[3,4]]"
        );
    }
    #[test]
    fn test_from_str_basic() {
        assert_eq!(
            Number::new(Number::Literal(1), Number::Literal(2)),
            Number::from_str("[1,2]").unwrap()
        );
    }

    #[test]
    fn test_from_str_intermediate() {
        let expected = Number::Pair(
            Box::new(Number::Pair(
                Box::new(Number::Literal(1)),
                Box::new(Number::Literal(2)),
            )),
            Box::new(Number::Literal(3)),
        );
        assert_eq!(Number::from_str("[[1,2],3]").unwrap(), expected);
    }

    #[test]
    fn test_from_str_intermediate_reverse() {
        let expected = Number::Pair(
            Box::new(Number::Literal(1)),
            Box::new(Number::Pair(
                Box::new(Number::Literal(2)),
                Box::new(Number::Literal(3)),
            )),
        );
        assert_eq!(Number::from_str("[1,[2,3]").unwrap(), expected);
    }

    #[test]
    fn test_from_str_advanced() {
        let expected = Number::Pair(
            Box::new(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Pair(
                        Box::new(Number::Literal(1)),
                        Box::new(Number::Literal(2)),
                    )),
                    Box::new(Number::Pair(
                        Box::new(Number::Literal(3)),
                        Box::new(Number::Literal(4)),
                    )),
                )),
                Box::new(Number::Pair(
                    Box::new(Number::Pair(
                        Box::new(Number::Literal(5)),
                        Box::new(Number::Literal(6)),
                    )),
                    Box::new(Number::Pair(
                        Box::new(Number::Literal(7)),
                        Box::new(Number::Literal(8)),
                    )),
                )),
            )),
            Box::new(Number::Literal(9)),
        );
        assert_eq!(
            Number::from_str("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").unwrap(),
            expected
        );
    }

    #[test]
    fn test_get_type() {
        let n1 = Number::from_str("[[1,2],3]").unwrap();
        let n2 = Number::Literal(0);
        assert!(n1.is_pair());
        assert!(!n1.is_literal());
        assert!(!n2.is_pair());
        assert!(n2.is_literal());
    }

    #[test]
    fn test_unwrap() {
        assert_eq!(Number::Literal(5).unwrap(), 5);
    }

    #[test]
    #[should_panic]
    fn test_failing_unwrap() {
        Number::Pair(Box::new(Number::Literal(0)), Box::new(Number::Literal(1))).unwrap();
    }

    #[test]
    fn test_explode_left() {
        let n = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap().explode();
        assert_eq!(n.to_string(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_explode_right() {
        let n = Number::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap().explode();
        assert_eq!(n.to_string(), "[7,[6,[5,[7,0]]]]");
    }
}

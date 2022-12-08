pub struct Chunk {
    line: String,
}

impl Chunk {
    pub fn new(pattern: &str) -> Self {
        Chunk {
            line: String::from(pattern),
        }
    }

    fn is_closing_char(c: &char) -> bool{
        ")]}>".contains(*c)
    }

    fn char_score(c: &char) -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        }
    }

    pub fn score(&self) -> usize {
        let mut opened = vec![];
        for c in self.line.chars() {
            if Chunk::is_closing_char(&c)  {
                match (opened.pop(), c) {
                    (Some(x), ')') if x == '('=> {},
                    (Some(x), '}') if x == '{'=> {},
                    (Some(x), ']') if x == '['=> {},
                    (Some(x), '>') if x == '<'=> {},
                    (_, _) => return Chunk::char_score(&c)
                }
            }
            else {
                opened.push(c)
            }
        }
        0
    }

    pub fn completion_string(&self) -> String {
        // Assume self is only incomplete, not corrupted
        let mut opened = String::new();
        for c in self.line.chars() {
            if Chunk::is_closing_char(&c)  {
                opened.pop();
            }
            else {
                opened.push(c)
            }
        }
        opened.chars().rev().map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unexpected char found !")
        }).collect::<String>()
    }

    pub fn completion_score(&self) -> usize {
        if self.score() != 0 {
            return 0;
        }
        self.completion_string().chars().fold(0, |acc, c| {
            5 * acc + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Chunk;
    #[test]
    fn test_get_score_0() {
        assert_eq!(Chunk::new("").score(), 0);
        assert_eq!(Chunk::new("([])").score(), 0);
        assert_eq!(Chunk::new("{()()()}").score(), 0);
        assert_eq!(Chunk::new("<([{}])>").score(), 0);
        assert_eq!(Chunk::new("[<>({}){}[([])<>]]").score(), 0);
        assert_eq!(Chunk::new("(((((((((())))))))))").score(), 0);
    }
    
    #[test]
    fn test_get_score_3() {
        assert_eq!(Chunk::new("[[<[([]))<([[{}[[()]]]").score(), 3);
    }

    #[test]
    fn test_get_score_57() {
        assert_eq!(Chunk::new("[{[{({}]{}}([{[{{{}}([]").score(), 57);
    }

    #[test]
    fn test_get_score_1197() {
        assert_eq!(Chunk::new("{([(<{}[<>[]}>{[]{[(<()>").score(), 1197);
    }

    #[test]
    fn test_get_score_25137() {
        assert_eq!(Chunk::new("<{([([[(<>()){}]>(<<{{").score(), 25137);
    }

    #[test]
    fn test_completion_string() {
        assert_eq!(Chunk::new("[({(<(())[]>[[{[]{<()<>>").completion_string(), "}}]])})]");
        assert_eq!(Chunk::new("[(()[<>])]({[<{<<[]>>(").completion_string(), ")}>]})");
        assert_eq!(Chunk::new("(((({<>}<{<{<>}{[]{[]{}").completion_string(), "}}>}>))))");
        assert_eq!(Chunk::new("{<[[]]>}<{[{[{[]{()[[[]").completion_string(), "]]}}]}]}>");
        assert_eq!(Chunk::new("<{([{{}}[<[[[<>{}]]]>[]]").completion_string(), "])}>");
    }
    
    #[test]
    fn test_completion_score() {
        assert_eq!(Chunk::new("[({(<(())[]>[[{[]{<()<>>").completion_score(), 288957);
        assert_eq!(Chunk::new("[(()[<>])]({[<{<<[]>>(").completion_score(), 5566);
        assert_eq!(Chunk::new("(((({<>}<{<{<>}{[]{[]{}").completion_score(), 1480781);
        assert_eq!(Chunk::new("{<[[]]>}<{[{[{[]{()[[[]").completion_score(), 995444);
        assert_eq!(Chunk::new("<{([{{}}[<[[[<>{}]]]>[]]").completion_score(), 294);
    }

    #[test]
    fn test_completion_score_corrupted() {
        assert_eq!(Chunk::new("[[<[([]))<([[{}[[()]]]").completion_score(), 0);
        assert_eq!(Chunk::new("[{[{({}]{}}([{[{{{}}([]").completion_score(), 0);
        assert_eq!(Chunk::new("{([(<{}[<>[]}>{[]{[(<()>").completion_score(), 0);
        assert_eq!(Chunk::new("<{([([[(<>()){}]>(<<{{").completion_score(), 0);
    }
}

mod chunk;
use chunk::Chunk;
pub struct Nav {
    chunks: Vec<Chunk>,
}

impl Nav {
    pub fn new(input: Vec<String>) -> Self {
        Nav {
            chunks: input.into_iter().map(|x| Chunk::new(&x)).collect(),
        }
    }

    pub fn score(&self) -> usize {
        self.chunks.iter().map(|x| x.score()).sum()
    }

    pub fn completion_score(&self) -> usize {
        let mut scores= self.chunks.iter().filter_map(
            |x| { let score = x.completion_score(); if score != 0 {Some(score)} else { None}}
        ).collect::<Vec<usize>>();
        scores.sort();
        scores[scores.len()/2]
    }
}

#[cfg(test)]
mod tests {
    use super::Nav;

    #[test]
    fn test_nav_score() {
        let data = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];
        assert_eq!(Nav::new(data).score(), 26397)
    }
    
    #[test]
    fn test_completion_score() {
        let data = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];
        assert_eq!(Nav::new(data).completion_score(), 288957)
    }
}

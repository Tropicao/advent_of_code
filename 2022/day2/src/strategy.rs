#[derive(PartialEq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

fn parse_round(round: &str) -> (Play, Play) {
    let mut raw = round.split_ascii_whitespace();
    let his = match raw.next() {
        Some("A") => Play::Rock,
        Some("B") => Play::Paper,
        Some("C") => Play::Scissor,
        x => panic!("Invalid play {:?}", x),
    };
    let my = match raw.next() {
        Some("X") => Play::Rock,
        Some("Y") => Play::Paper,
        Some("Z") => Play::Scissor,
        x => panic!("Invalid play {:?}", x),
    };
    (his, my)
}

fn parse_real_round(round: &str) -> (Play, Play) {
    let mut raw = round.split_ascii_whitespace();
    let his = match raw.next() {
        Some("A") => Play::Rock,
        Some("B") => Play::Paper,
        Some("C") => Play::Scissor,
        x => panic!("Invalid play {:?}", x),
    };
    let my = match raw.next() {
        Some("X") => match his {
            Play::Rock => Play::Scissor,
            Play::Paper => Play::Rock,
            Play::Scissor => Play::Paper,
        }
        Some("Y") => his,
        Some("Z") => match his {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissor,
            Play::Scissor => Play::Rock,
        },
        x => panic!("Invalid play {:?}", x),
    };
    (his, my)
}

fn play_score(play: Play) -> usize {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissor => 3,
    }
}

fn round_score(his: Play, my: Play) -> usize {
    match (his, my) {
        (Play::Rock, Play::Paper) | (Play::Scissor, Play::Rock) | (Play::Paper, Play::Scissor) => 6,
        (x, y) if x == y => 3,
        _ => 0,
    }
}

pub fn compute_score(input: &str) -> usize {
    let mut score = 0;
    for i in input.lines() {
        let (his, my) = parse_round(i);
        score += play_score(my) + round_score(his, my);
    }
    score
}

pub fn compute_real_score(input: &str) -> usize {
    let mut score = 0;
    for i in input.lines() {
        let (his, my) = parse_real_round(i);
        score += play_score(my) + round_score(his, my);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";
    #[test]
    fn test_score() {
        let score = compute_score(INPUT);
        assert_eq!(score, 15)
    }

    #[test]
    fn test_real_score() {
        let score = compute_real_score(INPUT);
        assert_eq!(score, 12)
    }
}

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 38,
        _ => panic!("Unexpected char {c}"),
    }
}

pub fn compute_priorities_sum(input: &str) -> usize {
    let mut priorities = 0;
    for line in input.lines() {
        let duplicate = line[..line.len() / 2]
            .chars()
            .find(|x| line[line.len() / 2..].contains(*x))
            .expect("No duplicate found");
        let priority = priority(duplicate);
        priorities += priority;
    }
    priorities
}

pub fn compute_badges_sum(input: &str) -> usize {
    let mut iter = input.lines();
    let mut priorities = 0;
    loop {
        match iter.next() {
            None => break,
            x => {
                let second_sack = iter.next();
                let third_sack = iter.next();
                let badge = x
                    .expect("No line available")
                    .chars()
                    .find(|x| second_sack.unwrap().contains(*x) && third_sack.unwrap().contains(*x))
                    .unwrap();
                priorities += priority(badge);
            }
        }
    }
    priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn compute_total_priority() {
        assert_eq!(compute_priorities_sum(INPUT), 157)
    }

    #[test]
    fn compute_badges_priority() {
        assert_eq!(compute_badges_sum(INPUT), 70)
    }
}

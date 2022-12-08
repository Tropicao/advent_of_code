use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::RangeBounds;

pub struct Input {
    digits: Vec<String>,
    segments: Vec<String>,
}

impl Input {
    pub fn new(raw_input: &str) -> Self {
        let mut parts = raw_input.trim().split('|');
        Input {
            digits: parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| String::from(x))
                .collect::<Vec<String>>(),
            segments: parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| String::from(x))
                .collect::<Vec<String>>(),
        }
    }

    pub fn count_special_digits(&self) -> usize {
        self.segments
            .iter()
            .filter(|&x| [2, 3, 4, 7].contains(&x.len()))
            .count()
    }

    pub fn guess_segments(&self) -> HashMap<u8, char> {
        let mut result = HashMap::<u8, char>::new();

        // Guess obvious digits
        let one = self.digits.iter().filter(|x| x.len() == 2).next().unwrap();
        let four = self.digits.iter().filter(|x| x.len() == 4).next().unwrap();
        let seven = self.digits.iter().filter(|x| x.len() == 3).next().unwrap();

        // Guess top segment which is present in 7 and not in 1
        result.insert(
            0,
            seven.chars().filter(|x| !one.contains(*x)).next().unwrap(),
        );

        // 0, 6 and 9 (all 6 segments) all have one of 1 segments, but one is missing 1 of one segment
        let six_segments = self
            .digits
            .iter()
            .filter(|&x| x.len() == 6)
            .collect::<Vec<&String>>();
        for segment in one.chars() {
            if six_segments.iter().filter(|&x| x.contains(segment)).count() == 2 {
                result.insert(2, segment);
                result.insert(5, one.chars().filter(|x| x != &segment).next().unwrap());
            }
        }

        // We now that 2 is the only one not containing the 5th segment
        let two = self
            .digits
            .iter()
            .filter(|&x| !x.contains(result[&5]))
            .next()
            .unwrap();

        // We know that upper left segment does not appear in 2 neither one
        result.insert(
            1,
            "abcdefg"
                .chars()
                .filter(|&x| !two.contains(x) && !one.contains(x))
                .next()
                .unwrap(),
        );

        // We know that all segments of 4 are now known except one, and that one is the center one
        result.insert(
            3,
            four.chars()
                .filter(|&x| !result.values().any(|&y| y == x))
                .next()
                .unwrap(),
        );

        // We now that 6 is the only 6 segments digit having only one of 1 digits
        let six = six_segments
            .iter()
            .filter(|&x| !one.chars().all(|c| x.contains(c)))
            .next()
            .unwrap();

        // We know that 9 is the six segments digit which is not 6 and not the one with the center segment
        let nine = six_segments
            .iter()
            .filter(|&x| !six.chars().all(|c| x.contains(c)))
            .filter(|&x| x.contains(result[&3]))
            .next()
            .unwrap();

        // We know that the only segment absent from 9 is the lower left
        result.insert(
            4,
            "abcdefg"
                .chars()
                .filter(|&x| !nine.contains(x))
                .next()
                .unwrap(),
        );

        // We know that the only unidentified segment is the lower one
        result.insert(
            6,
            "abcdefg"
                .chars()
                .filter(|&x| !result.values().any(|&c| x == c))
                .next()
                .unwrap(),
        );

        result
    }

    pub fn guess_digit_from_map(digit: &str, map: &HashMap<u8, char>) -> u32 {
        let digit_map: HashMap<u32, Vec<u8>> = HashMap::from([
            (0, vec![0, 1, 2, 4, 5, 6]),
            (1, vec![2, 5]),
            (2, vec![0, 2, 3, 4, 6]),
            (3, vec![0, 2, 3, 5, 6]),
            (4, vec![1, 2, 3, 5]),
            (5, vec![0, 1, 3, 5, 6]),
            (6, vec![0, 1, 3, 4, 5, 6]),
            (7, vec![0, 2, 5]),
            (8, vec![0, 1, 2, 3, 4, 5, 6]),
            (9, vec![0, 1, 2, 3, 5, 6]),
        ]);
        let indexes = map
            .iter()
            .filter_map(|(&k, &v)| if digit.contains(v) { Some(k) } else { None })
            .collect::<Vec<u8>>();
        digit_map
            .iter()
            .filter(|(_, v)| v.len() == indexes.len() && indexes.iter().all(|x| v.contains(x)))
            .next()
            .unwrap()
            .0
            .to_owned()
    }

    pub fn guess_display(&self) -> u32 {
        let segments_map = self.guess_segments();
        self.segments
            .iter()
            .enumerate()
            .map(|(i, v)| {
                Input::guess_digit_from_map(v, &segments_map)
                    * 10u32.pow((self.segments.len() - i-1).try_into().unwrap())
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Input;
    #[test]
    fn test_new_input() {
        let raw_data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let input = Input::new(raw_data);
        assert_eq!(
            input.digits,
            vec![
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb"
            ]
        );
        assert_eq!(input.segments, vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]);
    }

    #[test]
    fn test_count_special_digits() {
        let raw_data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let input = Input::new(raw_data);
        assert_eq!(input.count_special_digits(), 2);
    }

    #[test]
    fn test_guess_segment() {
        let raw_data =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let expected = HashMap::from([
            (0, 'd'),
            (1, 'e'),
            (2, 'a'),
            (3, 'f'),
            (4, 'g'),
            (5, 'b'),
            (6, 'c'),
        ]);
        let input = Input::new(raw_data);
        assert_eq!(input.guess_segments(), expected);
    }

    #[test]
    fn test_guess_digit_from_map() {
        let map = HashMap::from([
            (0, 'd'),
            (1, 'e'),
            (2, 'a'),
            (3, 'f'),
            (4, 'g'),
            (5, 'b'),
            (6, 'c'),
        ]);

        assert_eq!(Input::guess_digit_from_map("acedgfb", &map), 8);
        assert_eq!(Input::guess_digit_from_map("cdfbe", &map), 5);
        assert_eq!(Input::guess_digit_from_map("gcdfa", &map), 2);
        assert_eq!(Input::guess_digit_from_map("fbcad", &map), 3);
        assert_eq!(Input::guess_digit_from_map("dab", &map), 7);
        assert_eq!(Input::guess_digit_from_map("cefabd", &map), 9);
        assert_eq!(Input::guess_digit_from_map("cdfgeb", &map), 6);
        assert_eq!(Input::guess_digit_from_map("eafb", &map), 4);
        assert_eq!(Input::guess_digit_from_map("cagedb", &map), 0);
        assert_eq!(Input::guess_digit_from_map("ab", &map), 1);

    }

    #[test]
    fn test_guess_display() {
        let raw_data =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = Input::new(raw_data);
        assert_eq!(input.guess_display(), 5353);
    }
}

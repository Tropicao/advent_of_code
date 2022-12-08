pub fn search_pattern(input: &str, pattern_size:usize) -> usize
{
    let signal = input.chars().collect::<Vec<char>>();
    for i in 0..signal.len()-pattern_size {
        let mut part = Vec::from(&signal[i..i+pattern_size]);
        part.sort();
        part.dedup();
        if part.len() == pattern_size {
            return i+pattern_size;
        }
    }
    0
}

pub fn part_1(input: &str) -> usize {
    search_pattern(input, 4)
}

pub fn part_2(input: &str) -> usize {
    search_pattern(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Set test input in this variable
    const TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    struct TestData {
        input: &'static str,
        output_part_1: usize,
        output_part_2: usize
    }

    const TEST_DATA: [TestData;5] = [
        TestData {
            input: TEST_INPUT_1,
            output_part_1: 7,
            output_part_2: 19
        },
        TestData {
            input: TEST_INPUT_2,
            output_part_1: 5,
            output_part_2: 23
        },
        TestData {
            input: TEST_INPUT_3,
            output_part_1: 6,
            output_part_2: 23
        },
        TestData {
            input: TEST_INPUT_4,
            output_part_1: 10,
            output_part_2: 29
        },
        TestData {
            input: TEST_INPUT_5,
            output_part_1: 11,
            output_part_2: 26
        }
    ];

    #[test]
    fn test_part_1() {
        for test_data in TEST_DATA {
            assert_eq!(part_1(test_data.input), test_data.output_part_1)
        }
    }
    
    #[test]
    fn test_part_2() {
        for test_data in TEST_DATA {
            assert_eq!(part_2(test_data.input), test_data.output_part_2)
        }
    }
}

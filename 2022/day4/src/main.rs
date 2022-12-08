use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Containing pairs : {}", count_containing_pairs(&input));
    println!("Overlapping pairs : {}", count_overlaps(&input));
}

fn get_numeric_ids(line: &str) -> Vec<Vec<u8>> {
    line
    .split(',')
    .map(|x| {
        x.split('-')
            .map(|y| y.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    })
    .collect()
}

fn contains(raw_ids: Vec<Vec<u8>>) -> bool {
    raw_ids[0][0] >= raw_ids[1][0] && raw_ids[0][1] <= raw_ids[1][1]
    || raw_ids[0][0] <= raw_ids[1][0] && raw_ids[0][1] >= raw_ids[1][1]
}

fn overlaps(raw_ids: Vec<Vec<u8>>) -> bool {
    !(raw_ids[0][0] > raw_ids[1][1] || raw_ids[0][1] < raw_ids[1][0])
}

pub fn count_containing_pairs(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let raw_ids: Vec<Vec<u8>> = get_numeric_ids(line);
        match contains(raw_ids) 
        {
            true => sum += 1,
            _ => continue
        }
    }
    sum
}

pub fn count_overlaps(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let raw_ids: Vec<Vec<u8>> = get_numeric_ids(line);
        match overlaps(raw_ids) 
        {
            true => sum += 1,
            _ => continue
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn test_count_containing_pairs() {
        assert_eq!(count_containing_pairs(INPUT), 2);
    }
    #[test]
    fn test_count_overlaps() {
        assert_eq!(count_overlaps(INPUT), 4);
    }
}

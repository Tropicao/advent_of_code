mod lanternfish;
use lanternfish::Lanternfish;
pub struct Sea {
    fishes: Vec<u64>
}

impl Sea {
    pub fn new(fishes_timers: Vec<u64>) -> Self {
        let mut fishes = vec![0;9];
        for timer_start in fishes_timers {
            fishes[timer_start as usize] += 1;
        }
        Sea {
            fishes
        }
    }

    pub fn count_fishes_after(&self, days: u32) -> u64 {
        let mut result = self.fishes.clone();
        for _ in 0..days {
            let fish0 = result[0];
            for i in 0..8 {
                result[i] = result[i+1];
            }
            result[6] += fish0;
            result[8] = fish0;
        }
        result.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Sea;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn build_new_sea() -> Sea {
        let mut reader = BufReader::new(File::open("inputs_test.txt").unwrap());
        let mut raw_input = String::new();
        reader
            .read_line(&mut raw_input)
            .expect("Did not manage to read line");
        Sea::new(raw_input.split(',').map(|x| x.parse().unwrap()).collect())
    }

    #[test]
    fn test_new_sea() {
        let sea = Sea::new(vec![3]);
        assert_eq!(sea.count_fishes_after(0), 1);
    }

    #[test]
    fn test_after_18_days() {
        let sea = Sea::new(vec![3]);
        assert_eq!(sea.count_fishes_after(18), 5)
    }

    #[test]
    fn test_after_18_days_multiple_fishes() {
        let sea = build_new_sea();
        assert_eq!(sea.count_fishes_after(18), 26)
    }

    #[test]
    fn test_after_80_days() {
        let sea = build_new_sea();
        assert_eq!(sea.count_fishes_after(80), 5934)
    }

    #[test]
    fn test_after_256_days() {
        let sea = build_new_sea();
        assert_eq!(sea.count_fishes_after(256), 26984457539)
    }
}

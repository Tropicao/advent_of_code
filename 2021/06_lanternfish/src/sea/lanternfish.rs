#[derive(PartialEq, Debug)]
pub struct Lanternfish {
    timer: u32,
}

impl Lanternfish {
    pub fn new(timer: u32) -> Self {
        Lanternfish { timer }
    }

    pub fn count_childs_after(&self, days: u32) -> Vec<u32> {
        if days < self.timer + 1 {
            return vec![];
        }
        let days = days - self.timer - 1;
        (0..days / 7 + 1).map(|x| x * 7 + self.timer + 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Lanternfish;

    #[test]
    fn test_no_new_fish() {
        let fish = Lanternfish::new(3);
        assert_eq!(fish.count_childs_after(3), vec![]);
    }

    #[test]
    fn test_no_new_fish_bis() {
        let fish = Lanternfish::new(8);
        assert_eq!(fish.count_childs_after(8), vec![]);
    }

    #[test]
    fn test_one_new_fish() {
        let fish = Lanternfish::new(3);
        assert_eq!(fish.count_childs_after(4), vec![4]);
    }

    #[test]
    fn test_two_new_fishes() {
        let fish = Lanternfish::new(3);
        assert_eq!(fish.count_childs_after(11), vec![4, 11]);
    }

    #[test]
    fn test_three_new_fishes() {
        let fish = Lanternfish::new(3);
        assert_eq!(fish.count_childs_after(18), vec![4, 11, 18]);
    }

    #[test]
    fn test_newborn_no_new_fish() {
        // Assume a fish is born on day 4. It must not give birth before 13th day
        let birth_date = 4;
        let fish = Lanternfish::new(8);
        assert_eq!(fish.count_childs_after(12 - birth_date), vec![]);
    }

    #[test]
    fn test_newborn_one_new_fish() {
        // Assume a fish is born on day 4. It must give birth on 13th day
        let fish = Lanternfish::new(8);
        assert_eq!(fish.count_childs_after(9), vec![9]);
    }

    #[test]
    fn test_newborn_one_new_fish_bis() {
        // Assume a fish is born on day 4. It must give birth on 13th day
        let fish = Lanternfish::new(8);
        assert_eq!(fish.count_childs_after(15), vec![9]);
    }

    #[test]
    fn test_newborn_two_new_fish() {
        // Assume a fish is born on day 4. It must give birth on 13th day
        let fish = Lanternfish::new(8);
        assert_eq!(fish.count_childs_after(16), vec![9, 16]);
    }
}

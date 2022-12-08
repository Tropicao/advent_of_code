use std::io::Error;

pub struct FoodList {
    elves: Vec<Vec<usize>>,
}

impl FoodList {
    pub fn new(input: &str) -> Result<Self, Error> {
        let calories = input
            .split_terminator("\n\n")
            .into_iter()
            .map(|x| x.split("\n").map(|y| y.parse::<usize>().unwrap()).collect())
            .collect::<Vec<Vec<usize>>>();
        Ok(FoodList { elves: calories })
    }

    pub fn get_most_calories(&self) -> usize {
        let mut calories: Vec<usize> = self.elves
            .iter()
            .map(|x| x.iter().sum()).collect();
        calories.sort();
        calories[calories.len()-1]
    }

    pub fn get_top_three_calories(&self) -> usize {
        let mut calories: Vec<usize> = self.elves
            .iter()
            .map(|x| x.iter().sum()).collect();
        calories.sort();
        calories[calories.len()-3..calories.len()].iter().sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::FoodList;

    const TEST_LIST: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    
    #[test]
    fn test_calorie_count() {
        let list = FoodList::new(TEST_LIST).unwrap();
        assert_eq!(list.get_most_calories(), 24000);
        
    }

    #[test]
    fn test_top_three_calorie_count() {
        let list = FoodList::new(TEST_LIST).unwrap();
        assert_eq!(list.get_top_three_calories(), 45000);
        
    }
}

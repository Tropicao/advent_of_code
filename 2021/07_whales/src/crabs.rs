pub struct Crabs {
    positions: Vec<u32>
}

use std::cmp::Ordering;

impl Crabs {
    pub fn new(crabs: Vec<u32>) -> Self {
        let mut positions = crabs;
        positions.sort();
        Crabs {
            positions
        }
    }

    pub fn median(&self) -> u32 {
        self.positions[self.positions.len()/2]
    }

    pub fn mean(&self) -> u32 {
        let sum:u32 = self.positions.iter().sum();
        sum/self.positions.len() as u32 + 1
    }

    pub fn moving_cost(&self) -> u32 {
        let median = self.median();
        self.positions.iter().map(|x| match x.cmp(&median) {
            Ordering::Greater => x-median,
            _=>median-x
        }).sum()
    }
    
    pub  fn moving_cost_increasing(&self) -> u32 {
        let mean = self.mean();
        self.positions.iter().map(|x| match x.cmp(&mean) {
            Ordering::Greater => (1..=x-mean).sum::<u32>(),
            _=>(1..=mean-x).sum::<u32>()
        }).sum()
    }
    
    pub  fn moving_cost_from_target(&self, target: u32) -> u32 {
        self.positions.iter().map(|x| match x.cmp(&target) {
            Ordering::Greater => (1..=x-target).sum::<u32>(),
            _=>(1..=target-x).sum::<u32>()
        }).sum()
    }

    pub fn min(&self) -> u32 {
        self.positions[0]
    }

    pub fn max(&self) -> u32 {
        self.positions[self.positions.len()-1]
    }
}

#[cfg(test)]
mod tests
{
    use super::Crabs;
    #[test]
    fn test_median() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.median(), 2)
    }

    #[test]
    fn test_mean() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.mean(), 5)
    }
    
    #[test]
    fn test_min() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.min(), 0)
    }
    
    #[test]
    fn test_max() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.max(), 16)
    }

    #[test]
    fn test_cost() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.moving_cost(), 37)
    }

    #[test]
    fn test_increasing_cost() {
        let crabs = Crabs::new(vec![16,1,2,0,4,2,7,1,2,14]);
        assert_eq!(crabs.moving_cost_increasing(), 168)
    }
}
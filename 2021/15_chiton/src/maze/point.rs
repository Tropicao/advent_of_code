use std::cmp::Ordering;
#[derive(Clone)]
pub struct Point {
    selected: bool,
    cost: usize,
}

impl Point {
    pub fn new(cost: usize) -> Self {
        Point {
            selected: false,
            cost,
        }
    }

    pub fn selected(&self) -> bool {
        self.selected
    }
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn cost(&self) -> usize {
        self.cost
    }

    pub fn set_cost(&mut self, new_cost:usize) {
        self.cost = new_cost;
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Point {

}

#[cfg(test)]
mod tests {
    use super::Point;
    #[test]
    fn test_new() {
        let point = Point::new(5);
        assert_eq!(point.selected(), false);
        assert_eq!(point.cost(), 5);
    }
    #[test]
    fn test_select() {
        let mut point = Point::new(5);
        point.select();
        assert_eq!(point.selected(), true);
    }
    #[test]
    fn test_compare() {
        let a = Point::new(2);
        let b = Point::new(3);
        let c = Point::new(4);
        let d = Point::new(4);
        assert!(a<b);
        assert!(a<=b);
        assert!(a<c);
        assert!(a<=c);
        assert!(b<c);
        assert!(b<=c);
        assert!(c==d);
        assert!(c<=d);
    }
    
    #[test]
    fn test_set_cost() {
        let mut point = Point::new(3);
        point.set_cost(5);
        assert_eq!(point.cost(), 5);
        
    }
}

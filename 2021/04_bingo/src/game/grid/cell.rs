#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Cell {
    value: u32,
    checked: bool,
}

impl Cell {
    pub fn new(value: u32) -> Self {
        Cell {
            value,
            checked: false,
        }
    }

    pub fn check(&mut self) {
        self.checked = true;
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
#[cfg(test)]
mod test {
    use super::Cell;
    #[test]
    fn test_new_cell() {
        assert_eq!(
            Cell {
                value: 1,
                checked: false
            },
            Cell::new(1)
        );
    }

    #[test]
    fn test_check() {
        let mut cell = Cell::new(1);
        cell.check();
        assert!(cell.is_checked())
    }

    #[test]
    fn test_value() {
        let cell = Cell::new(15);
        assert_eq!(cell.value(), 15);
    }
}

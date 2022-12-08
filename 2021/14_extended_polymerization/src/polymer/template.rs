#[derive(Debug, PartialEq)]
pub struct Template{
    pattern: String,
    insert: char
}

impl Template{
    pub fn new(pattern: &str, insert:char) -> Self {
        Template { pattern: String::from(pattern), insert }
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn insert(&self) -> char {
        self.insert
    }

    pub fn build(&self) -> Vec<String> {
        let mut letters = self.pattern.chars();
        let mut left = String::from(letters.next().unwrap());
        left.push(self.insert);
        let mut right = String::from(self.insert);
        right.push(letters.next().unwrap());
        vec![left, right]
    }
}

#[cfg(test)]
mod tests {
    use super::Template;

    #[test]
    fn test_new() {
        let template = Template::new("AB", 'C');
        assert_eq!(template.pattern(), "AB");
    }
    
    #[test]
    fn test_build() {
        let template = Template::new("AB", 'C');
        assert_eq!(template.build(), vec!["AC", "CB"]);
    }
}
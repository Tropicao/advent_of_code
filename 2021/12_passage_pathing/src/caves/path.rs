#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    nodes: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Path { nodes: vec![] }
    }

    pub fn push(mut self, new_node: &str) -> Self {
        self.nodes.push(String::from(new_node));
        self
    }

    pub fn nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn concat(mut self, b: Self) -> Self {
        self.nodes = [self.nodes, b.nodes().to_vec()].concat();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Path;
    #[test]
    fn test_new_path() {
        let expected: Vec<String> = vec![];
        assert_eq!(Path::new().nodes(), &expected);
    }
    #[test]
    fn test_path_push_one() {
        let expected: Vec<String> = vec![String::from("start")];
        let path = Path::new().push("start");
        assert_eq!(path.nodes(), &expected);
    }
    #[test]
    fn test_path_push_two() {
        let expected: Vec<String> = vec![String::from("start"), String::from("end")];
        let path = Path::new().push("start").push("end");
        assert_eq!(path.nodes(), &expected);
    }
    #[test]
    fn test_path_push_three() {
        let expected: Vec<String> = vec![
            String::from("start"),
            String::from("middle"),
            String::from("end"),
        ];
        let path = Path::new().push("start").push("middle").push("end");
        assert_eq!(path.nodes(), &expected);
    }
    #[test]
    fn test_clone() {
        let expected: Vec<String> = vec![String::from("start"), String::from("end")];
        let path = Path::new().push("start").push("end").clone();
        assert_eq!(path.nodes(), &expected);
    }
    #[test]
    fn test_concat() {
        let expected: Vec<String> = vec![String::from("start"), String::from("end")];
        let path = Path::new().push("start");
        let path2 = Path::new().push("end");
        assert_eq!(path.concat(path2).nodes(), &expected)
    }
}

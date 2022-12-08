mod path;
use path::Path;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Caves {
    links: Vec<Vec<String>>,
}

impl Caves {
    pub fn new(input: Vec<Vec<String>>) -> Self {
        Caves { links: input }
    }

    pub fn from_file(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let raw_input: Vec<Vec<String>> = reader
            .lines()
            .map(|x| {
                x.unwrap()
                    .split("-")
                    .map(|y| String::from(y))
                    .collect::<Vec<String>>()
            })
            .collect();
        Caves::new(raw_input)
    }

    fn get_link_dest_from_source(link: &Vec<String>, from: &str) -> String {
        if link[0] == from {
            link[1].clone()
        } else {
            link[0].clone()
        }
    }

    fn childs(&self, from: &str) -> Vec<String> {
        let mut result = vec![];
        for link in &self.links {
            if link.contains(&from.to_string()) {
                result.push(Caves::get_link_dest_from_source(&link, from));
            }
        }
        result
    }

    pub fn build_path(
        &self,
        from: &str,
        to: &str,
        visited: HashSet<String>,
        advanced: bool,
    ) -> Vec<Path> {
        let mut result = vec![];
        if from == to {
            return vec![Path::new().push(from)];
        }
        
        for child in self.childs(from) {
            let mut is_advanced = advanced;
            let mut new_visited = visited.clone();
            if child == "start"
            {
                continue;
            }
            if child.chars().all(|x| x.is_lowercase())
            {
                if new_visited.contains(&child) {
                    if  !is_advanced {
                        continue
                    }
                    else {
                        is_advanced = false;
                    }
                }
                else {
                    new_visited.insert(child.to_string());
                }
            }
            for path in self.build_path(&child, to, new_visited, is_advanced) {
                result.push(Path::new().push(from).concat(path));
            }
        }
        result
    }

    pub fn paths(&self, advanced: bool) -> Vec<Path> {
        self.build_path("start", "end", HashSet::new(), advanced)
    }

    pub fn paths_count(&self, advanced: bool) -> usize {
        self.paths(advanced).len()
    }
}

#[cfg(test)]
mod tests {
    use super::Caves;
    use super::Path;
    use std::fmt::Debug;

    fn assert_vec_eq<T>(a: Vec<T>, b: Vec<T>)
    where
        T: PartialEq + Debug,
    {
        if !(a.iter().all(|x| b.contains(&x)) && b.iter().all(|x| a.contains(&x))) {
            panic!("\nExpected : {:#?}\nGot : {:#?}", b, a);
        }
    }

    #[test]
    fn test_new_caves() {
        let caves = Caves::from_file("inputs_tests.txt");
        let expected = vec![
            vec![String::from("start"), String::from("A")],
            vec![String::from("start"), String::from("b")],
            vec![String::from("A"), String::from("c")],
            vec![String::from("A"), String::from("b")],
            vec![String::from("b"), String::from("d")],
            vec![String::from("A"), String::from("end")],
            vec![String::from("b"), String::from("end")],
        ];
        assert_vec_eq(caves.links, expected);
    }

    #[test]
    fn test_one_path() {
        let caves = Caves::new(vec![vec![String::from("start"), String::from("end")]]);
        let expected = vec![Path::new().push("start").push("end")];
        assert_vec_eq(caves.paths(false), expected);
    }

    #[test]
    fn test_one_path_with_one_middle_node() {
        let caves = Caves::new(vec![
            vec![String::from("start"), String::from("a")],
            vec![String::from("a"), String::from("end")],
        ]);
        let expected = vec![Path::new().push("start").push("a").push("end")];
        assert_vec_eq(caves.paths(false), expected);
    }

    #[test]
    fn test_one_path_with_two_middle_node() {
        let caves = Caves::new(vec![
            vec![String::from("start"), String::from("a")],
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("end")],
        ]);
        let expected = vec![Path::new().push("start").push("a").push("b").push("end")];
        assert_vec_eq(caves.paths(false), expected);
    }

    #[test]
    fn test_two_paths() {
        let caves = Caves::new(vec![
            vec![String::from("start"), String::from("a")],
            vec![String::from("start"), String::from("b")],
            vec![String::from("a"), String::from("end")],
            vec![String::from("b"), String::from("end")],
        ]);
        let expected = vec![
            Path::new().push("start").push("a").push("end"),
            Path::new().push("start").push("b").push("end"),
        ];
        assert_vec_eq(caves.paths(false), expected);
    }
    #[test]
    fn test_reusable_nodes_in_paths() {
        let caves = Caves::new(vec![
            vec![String::from("start"), String::from("A")],
            vec![String::from("A"), String::from("end")],
            vec![String::from("A"), String::from("b")],
        ]);
        let expected = vec![
            Path::new().push("start").push("A").push("end"),
            Path::new()
                .push("start")
                .push("A")
                .push("b")
                .push("A")
                .push("end"),
        ];
        assert_vec_eq(caves.paths(false), expected);
    }

    #[test]
    fn test_paths() {
        let caves = Caves::from_file("inputs_tests.txt");
        #[rustfmt::skip]
            let expected = vec![
                Path::new().push("start").push("A").push("b").push("A").push("c").push("A").push("end"),
                Path::new().push("start").push("A").push("b").push("A").push("end"),
                Path::new().push("start").push("A").push("b").push("end"),
                Path::new().push("start").push("A").push("c").push("A").push("b").push("A").push("end"),
                Path::new().push("start").push("A").push("c").push("A").push("b").push("end"),
                Path::new().push("start").push("A").push("c").push("A").push("end"),
                Path::new().push("start").push("A").push("end"),
                Path::new().push("start").push("b").push("A").push("c").push("A").push("end"),
                Path::new().push("start").push("b").push("A").push("end"),
                Path::new().push("start").push("b").push("end")
                ];
        assert_vec_eq(caves.paths(false), expected);
    }

    #[test]
    fn test_path_count() {
        let cave = Caves::from_file("inputs_tests.txt");
        assert_eq!(cave.paths_count(false), 10);
    }

    #[test]
    fn test_path_count_bis() {
        let cave = Caves::from_file("inputs_tests_bis.txt");
        assert_eq!(cave.paths_count(false), 19);
    }

    #[test]
    fn test_path_count_bis_bis() {
        let cave = Caves::from_file("inputs_tests_bis_bis.txt");
        assert_eq!(cave.paths_count(false), 226);
    }

    #[test]
    fn test_advanced_reusable_nodes_in_paths() {
        let caves = Caves::new(vec![
            vec![String::from("start"), String::from("a")],
            vec![String::from("a"), String::from("end")],
            vec![String::from("a"), String::from("b")],
        ]);
        let expected = vec![
            Path::new().push("start").push("a").push("end"),
            Path::new()
                .push("start")
                .push("a")
                .push("b")
                .push("a")
                .push("end"),
        ];
        assert_vec_eq(caves.paths(true), expected);
    }

    #[test]
    fn test_path_count_advanced() {
        let cave = Caves::from_file("inputs_tests.txt");
        for path in cave.paths(true) {
            println!("{:?}", path.nodes());
        }
        assert_eq!(cave.paths_count(true), 36);
    }

    #[test]
    fn test_path_count_bis_advanced() {
        let cave = Caves::from_file("inputs_tests_bis.txt");
        assert_eq!(cave.paths_count(true), 103);
    }

    #[test]
    fn test_path_count_bis_bis_advanced() {
        let cave = Caves::from_file("inputs_tests_bis_bis.txt");
        assert_eq!(cave.paths_count(true), 3509);
    }
}

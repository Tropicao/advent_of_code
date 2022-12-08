mod template;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use template::Template;
#[derive(Debug)]
pub struct Polymer {
    chain: HashMap<String, usize>,
    templates: Vec<Template>,
    counts: HashMap<char, usize>,
}

impl Polymer {
    pub fn from_file(path: &str) -> Self {
        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut temp = String::new();
        let mut chain = HashMap::new();
        let mut counts = HashMap::new();
        
        // Read initial chain
        reader
            .read_line(&mut temp)
            .expect("Did not manage to read initial chain");
        for chunck in temp
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|x| x.iter().collect::<String>())
        {
            let entry = chain.entry(chunck).or_insert(0);
            *entry += 1;
        }
        for c in temp.trim().chars() {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        reader
            .read_line(&mut temp)
            .expect("Did not manage to evacuate blank line");
        let mut templates = Vec::new();
        loop {
            temp.clear();
            match reader.read_line(&mut temp) {
                Ok(0) => break,
                Err(_) => panic!("Could not keep reading templates"),
                _ => {
                    let raw_template = temp.trim().split("->").collect::<Vec<&str>>();
                    templates.push(Template::new(
                        raw_template[0].trim(),
                        raw_template[1].trim().chars().next().unwrap(),
                    ))
                }
            }
        }
        Polymer {
            chain,
            templates,
            counts,
        }
    }

    pub fn grow(&mut self) {
        let result = self.chain.clone();
        // All current templates disappear during polymerisation
        for (_, v) in self.chain.iter_mut() {
            *v = 0;
        }
        for (k, &value) in result.iter() {
            let template = self.templates.iter().find(|&x| x.pattern() == k).unwrap();
            for new_pattern in template.build() {
                let entry = self.chain.entry(new_pattern.clone()).or_insert(0);
                *entry += value;
            }
            let new_count = self.counts.entry(template.insert()).or_insert(0);
            *new_count += value;
        }
        self.chain.retain(|_, &mut v| v > 0);
    }

    fn get_most(&self) -> usize {
        self.counts.iter().map(|(_, &v)| v).max().unwrap()
    }

    fn get_least(&self) -> usize {
        self.counts.iter().map(|(_, &v)| v).min().unwrap()
    }

    pub fn get_diff(&self) -> usize {
        self.get_most() - self.get_least()
    }
}

#[cfg(test)]
mod tests {
    use super::{Polymer, Template};
    use std::collections::HashMap;
    #[test]
    fn test_load_file() {
        let polymer = Polymer::from_file("inputs_test.txt");
        let expected_templates = vec![
            Template::new("CH", 'B'),
            Template::new("HH", 'N'),
            Template::new("CB", 'H'),
            Template::new("NH", 'C'),
            Template::new("HB", 'C'),
            Template::new("HC", 'B'),
            Template::new("HN", 'C'),
            Template::new("NN", 'C'),
            Template::new("BH", 'H'),
            Template::new("NC", 'B'),
            Template::new("NB", 'B'),
            Template::new("BN", 'B'),
            Template::new("BB", 'N'),
            Template::new("BC", 'B'),
            Template::new("CC", 'N'),
            Template::new("CN", 'C'),
        ];
        assert_eq!(polymer.templates, expected_templates);
    }

    #[test]
    fn test_count_at_start() {
        let polymer = Polymer::from_file("inputs_test.txt");
        assert_eq!(polymer.get_diff(), 1);
    }
    #[test]
    fn test_count_after_one() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        polymer.grow();
        assert_eq!(polymer.get_diff(), 1);
    }
    #[test]
    fn test_counts_after_one() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        let expected = HashMap::from([('N', 2), ('C', 2), ('B', 2), ('H', 1)]);
        polymer.grow();
        assert_eq!(polymer.counts, expected);
    }
    #[test]
    fn test_count_after_two() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        polymer.grow();
        polymer.grow();
        assert_eq!(polymer.get_diff(), 5);
    }
    #[test]
    fn test_counts_after_two() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        let expected = HashMap::from([('N', 2), ('C', 4), ('B', 6), ('H', 1)]);
        polymer.grow();
        polymer.grow();
        assert_eq!(polymer.counts, expected);
    }
    #[test]
    fn test_counts_after_three() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        let expected = HashMap::from([('N', 5), ('C', 5), ('B', 11), ('H', 4)]);
        polymer.grow();
        polymer.grow();
        polymer.grow();
        assert_eq!(polymer.counts, expected);
    }
    #[test]
    fn test_counts_after_four() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        let expected = HashMap::from([('N', 11), ('C', 10), ('B', 23), ('H', 5)]);
        polymer.grow();
        polymer.grow();
        polymer.grow();
        polymer.grow();
        assert_eq!(polymer.counts, expected);
    }
    #[test]
    fn test_get_most() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        for _ in 0..10 {
            polymer.grow();
        }
        assert_eq!(polymer.get_most(), 1749);
    }
    #[test]
    fn test_get_least() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        for _ in 0..10 {
            polymer.grow();
        }
        assert_eq!(polymer.get_least(), 161);
    }
    #[test]
    fn test_get_diff() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        for _ in 0..10 {
            polymer.grow();
        }
        assert_eq!(polymer.get_diff(), 1588)
    }
    #[test]
    fn test_get_diff_large() {
        let mut polymer = Polymer::from_file("inputs_test.txt");
        for _ in 0..40 {
            polymer.grow();
        }
        assert_eq!(polymer.get_diff(), 2188189693529)
    }
}

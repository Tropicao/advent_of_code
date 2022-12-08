use std::collections::VecDeque;

enum EntryType {
    File(String, usize),
    Directory(String),
}

struct Entry {
    entry: EntryType,
    children: Vec<Entry>,
}

impl Entry {
    fn new_file(name: &str, size: usize) -> Self {
        Entry {
            entry: EntryType::File(String::from(name), size),
            children: vec![],
        }
    }
    fn new_dir(name: &str) -> Self {
        Entry {
            entry: EntryType::Directory(String::from(name)),
            children: vec![],
        }
    }
    fn name(&self) -> &str {
        match &self.entry {
            EntryType::File(n, _) => n,
            EntryType::Directory(n) => n,
        }
    }

    fn get_total_size(&self) -> usize {
        if let EntryType::File(_, s) = self.entry {
            s
        } else {
            self.children.iter().map(|x| x.get_total_size()).sum()
        }
    }

    fn get_directories_size(&self) -> Vec<usize> {
        let mut result = vec![self.get_total_size()];
        let mut child_directories_sizes = self
            .children
            .iter()
            .filter_map(|x| {
                if let EntryType::Directory(_) = &x.entry {
                    Some(x.get_directories_size())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<usize>>();
        result.append(&mut child_directories_sizes);
        result
    }
}

fn walk<'a>(root: &'a mut Entry, lines: &'a mut VecDeque<&str>) {
    while let Some(line) = lines.pop_front() {
        let content: Vec<&str> = line.split_ascii_whitespace().collect();
        if content[0].starts_with('$') {
            match content[1] {
                "ls" => continue,
                "cd" => {
                    if content[2].starts_with("..") {
                        return;
                    }
                    let child_to_walk = root
                        .children
                        .iter_mut()
                        .find(|x| {
                            if let EntryType::Directory(n) = &x.entry {
                                n == content[2]
                            } else {
                                false
                            }
                        })
                        .unwrap();
                    walk(child_to_walk, lines)
                }
                _ => panic!("Unexpected command {}", content[1]),
            }
        } else if content[0] == "dir" {
            root.children.push(Entry::new_dir(content[1]));
        } else {
            root.children
                .push(Entry::new_file(content[1], content[0].parse().unwrap()));
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut root = Entry::new_dir("/");
    let mut lines: VecDeque<&str> = input.lines().skip(1).collect();
    walk(&mut root, &mut lines);
    root.get_directories_size()
        .into_iter()
        .filter(|&x| x < 100000)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    const TOTAL_SIZE: usize = 70000000;
    const OTA_NEEDED_SPACE: usize = 30000000;
    let mut root = Entry::new_dir("/");
    let mut lines: VecDeque<&str> = input.lines().skip(1).collect();
    walk(&mut root, &mut lines);
    let mut sizes = root.get_directories_size();
    sizes.sort();
    let unused_space = TOTAL_SIZE - sizes.pop().unwrap();
    let needed_space = OTA_NEEDED_SPACE - unused_space;
    *sizes.iter().find(|&x| x >= &needed_space).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Set test input in this variable
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        // Adjust part 1 test to match provided challenge example
        assert_eq!(part_1(TEST_INPUT), 95437)
    }

    #[test]
    fn test_part_2() {
        // Adjust part 2 test to match provided challenge example
        assert_eq!(part_2(TEST_INPUT), 24933642)
    }
}

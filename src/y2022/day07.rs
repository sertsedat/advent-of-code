use std::collections::HashMap;

#[derive(PartialEq)]
enum CommandLine {
    ChangeDirectory(String),
    List,
    File(usize),
    Directory(String),
}

impl From<&str> for CommandLine {
    fn from(s: &str) -> Self {
        if s.starts_with("$ ") {
            let command = s.split_once(" ").unwrap().1;

            match command {
                "ls" => Self::List,
                _ => {
                    let to = command.split_once(" ").unwrap().1.to_string();
                    Self::ChangeDirectory(to)
                }
            }
        }
        // outputs
        else if s.starts_with("dir") {
            let name = s.split_once(" ").unwrap().1.to_string();
            Self::Directory(name)
        } else {
            Self::File(s.split_once(" ").unwrap().0.parse().unwrap())
        }
    }
}

#[derive(Debug)]
enum Type {
    Directory(String),
    File(usize),
}

#[derive(Debug)]
pub struct FileSystem {
    contents: HashMap<String, Vec<Type>>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    fn size_of(&self, directory: &str) -> usize {
        let contents = self.contents.get(directory).unwrap();
        contents
            .iter()
            .map(|content| match content {
                Type::File(size) => *size,
                Type::Directory(child) => {
                    let directory = directory.to_string() + "/" + child;
                    self.size_of(&directory)
                }
            })
            .sum::<usize>()
    }
}

fn parse_commands_into_filesystem(input: &Vec<CommandLine>) -> FileSystem {
    // start from root / directory
    let mut directories: Vec<String> = vec!["/".to_string()];
    let mut file_system = FileSystem::new();

    for line in input.iter() {
        match line {
            CommandLine::ChangeDirectory(to) => {
                match to.as_str() {
                    ".." => {
                        directories.pop();
                    }
                    _ => directories.push(to.clone()),
                };
            }
            CommandLine::List => {}
            CommandLine::File(size) => {
                let current_dir = directories.join("/");
                file_system
                    .contents
                    .entry(current_dir)
                    .and_modify(|contents| contents.push(Type::File(*size)))
                    .or_insert_with(|| vec![Type::File(*size)]);
            }
            CommandLine::Directory(dir) => {
                let current_dir = directories.join("/");
                file_system
                    .contents
                    .entry(current_dir.clone())
                    .and_modify(|contents| contents.push(Type::Directory(dir.clone())))
                    .or_insert_with(|| vec![Type::Directory(dir.clone())]);
            }
        }
    }
    file_system
}

#[aoc_generator(day07)]
pub fn generate_input(input: &str) -> FileSystem {
    let command_lines: Vec<CommandLine> = input
        .lines()
        // skip the first line since we start from the root
        .skip(1)
        .map(CommandLine::from)
        .collect();

    parse_commands_into_filesystem(&command_lines)
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &FileSystem) -> usize {
    input
        .contents
        .keys()
        .map(|directory| input.size_of(&directory))
        .filter(|size| *size <= 100_000)
        .sum()
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &FileSystem) -> usize {
    let used_size = input.size_of("/");
    let max_size = 70_000_000 - 30_000_000;
    let smallest_to_delete = used_size - max_size;

    input
        .contents
        .keys()
        .map(|directory| input.size_of(directory))
        .filter(|size| *size >= smallest_to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> FileSystem {
        generate_input(
            "$ cd /
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
7214296 k",
        )
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 95437);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 24933642);
    }

    #[test]
    fn test_input_part1() {
        let input = generate_input(include_str!("../../input/2022/day7.txt"));
        let result = solve_part1(&input);

        assert_eq!(result, 1306611);
    }

    #[test]
    fn test_input_part2() {
        let input = generate_input(include_str!("../../input/2022/day7.txt"));
        let result = solve_part2(&input);

        assert_eq!(result, 13210366);
    }
}
